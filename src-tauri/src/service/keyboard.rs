use crate::prelude::*;
use crate::service::clipboard::get_last_clipboard_db;
use common::types::enums::ClipboardType;
use common::types::orm_query::FullClipboardDto;
#[cfg(not(target_os = "linux"))]
use enigo::{Enigo, Keyboard, Settings};
use std::time::Duration;

pub async fn type_last_clipboard() {
    let clipboard = get_last_clipboard_db().await;

    if let Ok(clipboard_data) = clipboard {
        if let Some(content) = get_clipboard_content(&clipboard_data) {
            if content.len() < 500 {
                std::thread::sleep(Duration::from_millis(300));
                type_text(&content);
            }
        }
    }
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
    use std::process::Command;

    let is_wayland = std::env::var_os("WAYLAND_DISPLAY").is_some()
        || std::env::var("XDG_SESSION_TYPE")
            .map(|v| v.eq_ignore_ascii_case("wayland"))
            .unwrap_or(false);

    let candidates: &[(&str, &[&str])] = if is_wayland {
        &[("wtype", &["--"]), ("ydotool", &["type", "--"])]
    } else {
        &[
            ("xdotool", &["type", "--clearmodifiers", "--"]),
            ("wtype", &["--"]),
        ]
    };

    for (cmd, prefix_args) in candidates {
        let mut command = Command::new(cmd);
        command.args(*prefix_args).arg(content);

        match command.status() {
            Ok(status) if status.success() => return,
            Ok(status) => {
                printlog!("type_clipboard: {cmd} exited with {status}");
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => {
                printlog!("type_clipboard: failed to spawn {cmd}: {e}");
            }
        }
    }

    printlog!(
        "type_clipboard: no working typing tool found (tried {}). Install xdotool (X11) or wtype/ydotool (Wayland).",
        candidates
            .iter()
            .map(|(c, _)| *c)
            .collect::<Vec<_>>()
            .join(", ")
    );
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
