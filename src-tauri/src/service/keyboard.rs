use crate::prelude::*;
use crate::service::cipher::is_encryption_key_set;
use crate::service::clipboard::{get_clipboard_db, get_last_clipboard_db};
use crate::service::decrypt::decrypt_clipboard;
use crate::service::settings::get_global_settings;
use crate::service::target_window::{
    current_window, get_target_window, modifiers_held, raise_target_window, TargetWindow,
};
use crate::tao::global::get_main_window;
use common::types::enums::{ClipboardType, PasteOnSelect};
use common::types::orm_query::FullClipboardDto;
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::time::Duration;
use uuid::Uuid;

// Mirrors CopyQ's window_wait_* defaults, which are tuned for the same job.
const WAIT_BEFORE_RAISE_MS: u64 = 20;
const WAIT_RAISED_MS: u64 = 150;
const WAIT_AFTER_RAISED_MS: u64 = 50;
const FOCUS_POLL_INTERVAL_MS: u64 = 10;

/// Some toolkits drop a chord that is pressed and released in the same frame.
const KEY_HOLD_MS: u64 = if cfg!(target_os = "macos") { 100 } else { 50 };

/// Used where the focused window cannot be queried, so settling is all we can do.
const BLIND_FOCUS_RETURN_MS: u64 = 300;

/// Budget for the user to release a hotkey before the paste is sent regardless.
const MODIFIER_RELEASE_TIMEOUT_MS: u64 = 2000;

pub async fn type_last_clipboard() {
    let clipboard = get_last_clipboard_db().await;

    if let Ok(clipboard_data) = clipboard {
        if let Some(content) = get_clipboard_content(&clipboard_data) {
            if content.len() < 500 {
                std::thread::sleep(Duration::from_millis(BLIND_FOCUS_RETURN_MS));
                type_text(&content);
            }
        }
    }
}

/// Sends the configured paste action to the window the user came from.
///
/// Does nothing unless the user opted in. Failures are logged rather than
/// surfaced: the clipboard write already succeeded by this point, so the entry
/// can still be pasted by hand.
pub async fn paste_on_select(clipboard_id: Uuid) {
    let mode = PasteOnSelect::from_setting(&get_global_settings().paste_on_select);

    if mode == PasteOnSelect::Off {
        return;
    }

    // Resolve the text before hiding, so an image or file entry in type mode
    // leaves the window alone instead of closing it for nothing.
    let text = if mode == PasteOnSelect::Type {
        match resolve_text(clipboard_id).await {
            Some(text) => Some(text),
            None => return,
        }
    } else {
        None
    };

    hide_for_paste();

    if !wait_for_target_focus().await {
        return;
    }

    wait_for_modifiers_released().await;

    match text {
        Some(text) => type_text(&text),
        None => send_paste_chord(),
    }
}

/// Pastes the clipboard into the focused window, for text produced by a command hotkey.
pub async fn paste_into_active_window() {
    if get_main_window().is_visible().unwrap_or(false) {
        hide_for_paste();
        if !wait_for_target_focus().await {
            return;
        }
    }

    wait_for_modifiers_released().await;
    send_paste_chord();
}

/// Waits for the user to let go of any modifier they are still holding.
///
/// Selection is usually driven by a global hotkey, so Ctrl or Super is often
/// still down. Injecting the chord on top of that produces a different
/// shortcut in the target window, or types the wrong character.
async fn wait_for_modifiers_released() {
    let deadline = std::time::Instant::now() + Duration::from_millis(MODIFIER_RELEASE_TIMEOUT_MS);

    while modifiers_held() {
        if std::time::Instant::now() >= deadline {
            printlog!("paste_on_select: modifiers still held, sending anyway");
            return;
        }

        tokio::time::sleep(Duration::from_millis(FOCUS_POLL_INTERVAL_MS)).await;
    }
}

async fn resolve_text(clipboard_id: Uuid) -> Option<String> {
    let mut clipboard_data = match get_clipboard_db(clipboard_id).await {
        Ok(data) => data,
        Err(e) => {
            printlog!("paste_on_select: failed to load clipboard: {e:?}");
            return None;
        }
    };

    if clipboard_data.clipboard.encrypted && is_encryption_key_set() {
        clipboard_data = match decrypt_clipboard(clipboard_data) {
            Ok(data) => data,
            Err(e) => {
                printlog!("paste_on_select: failed to decrypt clipboard: {e:?}");
                return None;
            }
        };
    }

    get_clipboard_content(&clipboard_data)
}

/// Unlike the copy path, this hide is not gated on the build profile: leaving
/// the window up means pasting into Clippy itself.
fn hide_for_paste() {
    // The resulting Focused(false) is a no-op because that handler returns
    // early once the window is already hidden.
    get_main_window().hide().ok();
}

/// Waits for the recorded window to regain focus, raising it if it does not.
///
/// Returns false when focus never arrives, in which case no keys are sent.
/// Pasting into an unknown window is worse than not pasting at all.
async fn wait_for_target_focus() -> bool {
    let Some(target) = get_target_window() else {
        // Either the platform cannot report a window or the picker was opened
        // from the tray. Raise what we can and settle; there is nothing to poll
        // against, so the delay is the only guarantee that focus has moved on.
        raise_target_window();
        tokio::time::sleep(Duration::from_millis(BLIND_FOCUS_RETURN_MS)).await;
        return true;
    };

    if poll_focus(target, WAIT_BEFORE_RAISE_MS).await {
        tokio::time::sleep(Duration::from_millis(WAIT_AFTER_RAISED_MS)).await;
        return true;
    }

    raise_target_window();

    if poll_focus(target, WAIT_RAISED_MS).await {
        tokio::time::sleep(Duration::from_millis(WAIT_AFTER_RAISED_MS)).await;
        return true;
    }

    printlog!("paste_on_select: window {target} never regained focus, not sending keys");
    false
}

async fn poll_focus(target: TargetWindow, timeout_ms: u64) -> bool {
    let deadline = std::time::Instant::now() + Duration::from_millis(timeout_ms);

    loop {
        if current_window() == Some(target) {
            return true;
        }

        if std::time::Instant::now() >= deadline {
            return false;
        }

        tokio::time::sleep(Duration::from_millis(FOCUS_POLL_INTERVAL_MS)).await;
    }
}

/// Ctrl+V (Cmd+V on macOS). Shift+Insert would also cover terminals, but on X11
/// most toolkits serve it from the PRIMARY selection rather than CLIPBOARD, and
/// Clippy only owns CLIPBOARD, so it pastes nothing.
fn send_paste_chord() {
    #[cfg(target_os = "macos")]
    let (modifier, key) = (Key::Meta, Key::Unicode('v'));
    #[cfg(not(target_os = "macos"))]
    let (modifier, key) = (Key::Control, Key::Unicode('v'));

    let mut enigo = match new_enigo() {
        Ok(enigo) => enigo,
        Err(e) => {
            printlog!("paste_on_select: Enigo::new failed: {e:?}");
            #[cfg(target_os = "linux")]
            send_paste_chord_fallback();
            return;
        }
    };

    if let Err(e) = enigo.key(modifier, Direction::Press) {
        printlog!("paste_on_select: modifier press failed: {e:?}");
        #[cfg(target_os = "linux")]
        send_paste_chord_fallback();
        return;
    }

    std::thread::sleep(Duration::from_millis(KEY_HOLD_MS));

    if let Err(e) = enigo.key(key, Direction::Click) {
        printlog!("paste_on_select: key click failed: {e:?}");
    }

    // Released unconditionally: a stuck modifier is worse than a missed paste.
    if let Err(e) = enigo.key(modifier, Direction::Release) {
        printlog!("paste_on_select: modifier release failed: {e:?}");
    }
}

fn new_enigo() -> Result<Enigo, enigo::NewConError> {
    Enigo::new(&Settings::default())
}

/// Used when enigo cannot reach the display server at all.
#[cfg(target_os = "linux")]
fn send_paste_chord_fallback() {
    let candidates: &[(&str, &[&str])] = if is_wayland() {
        &[
            ("wtype", &["-M", "ctrl", "v", "-m", "ctrl"]),
            // ydotool speaks raw evdev codes, not key names:
            // 29 = KEY_LEFTCTRL, 47 = KEY_V, :1 press, :0 release.
            ("ydotool", &["key", "29:1", "47:1", "47:0", "29:0"]),
        ]
    } else {
        &[
            ("xdotool", &["key", "--clearmodifiers", "ctrl+v"]),
            ("wtype", &["-M", "ctrl", "v", "-m", "ctrl"]),
        ]
    };

    run_first_available("paste_on_select", candidates, None);
}

#[cfg(target_os = "linux")]
fn is_wayland() -> bool {
    std::env::var_os("WAYLAND_DISPLAY").is_some()
        || std::env::var("XDG_SESSION_TYPE")
            .map(|v| v.eq_ignore_ascii_case("wayland"))
            .unwrap_or(false)
}

/// Runs the first candidate that exists and exits cleanly. `trailing` is
/// appended as a final argument when present.
#[cfg(target_os = "linux")]
fn run_first_available(what: &str, candidates: &[(&str, &[&str])], trailing: Option<&str>) -> bool {
    use std::process::Command;

    for (cmd, prefix_args) in candidates {
        let mut command = Command::new(cmd);
        command.args(*prefix_args);

        if let Some(trailing) = trailing {
            command.arg(trailing);
        }

        match command.status() {
            Ok(status) if status.success() => return true,
            Ok(status) => printlog!("{what}: {cmd} exited with {status}"),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => printlog!("{what}: failed to spawn {cmd}: {e}"),
        }
    }

    printlog!(
        "{what}: no working input tool found (tried {}). Install xdotool (X11) or wtype/ydotool (Wayland; ydotool also needs ydotoold running).",
        candidates
            .iter()
            .map(|(c, _)| *c)
            .collect::<Vec<_>>()
            .join(", ")
    );

    false
}

#[cfg(not(target_os = "linux"))]
fn type_text(content: &str) {
    match Enigo::new(&Settings::default()) {
        Ok(mut enigo) => {
            if let Err(e) = enigo.text(content) {
                printlog!("type_clipboard: enigo.text failed: {e:?}");
            }
        }
        Err(e) => printlog!("type_clipboard: Enigo::new failed: {e:?}"),
    }
}

#[cfg(target_os = "linux")]
fn type_text(content: &str) {
    let candidates: &[(&str, &[&str])] = if is_wayland() {
        &[("wtype", &["--"]), ("ydotool", &["type", "--"])]
    } else {
        &[
            ("xdotool", &["type", "--clearmodifiers", "--"]),
            ("wtype", &["--"]),
        ]
    };

    run_first_available("type_clipboard", candidates, Some(content));
}

fn get_clipboard_content(clipboard_data: &FullClipboardDto) -> Option<String> {
    let types = ClipboardType::from_json_value(&clipboard_data.clipboard.types)?;

    types.iter().find_map(|clipboard_type| {
        match clipboard_type {
            ClipboardType::Text => clipboard_data.text.as_ref().map(|model| model.data.clone()),
            ClipboardType::Html => clipboard_data
                .text
                .as_ref()
                .map(|model| model.data.clone())
                .or_else(|| clipboard_data.html.as_ref().map(|model| model.data.clone())),
            ClipboardType::Rtf => clipboard_data
                .text
                .as_ref()
                .map(|model| model.data.clone())
                .or_else(|| clipboard_data.rtf.as_ref().map(|model| model.data.clone())),
            // Skip Image and File types as they don't have string content
            _ => None,
        }
    })
}
