use crate::{
    printlog,
    service::{
        clipboard::copy_clipboard_from_index,
        global::{
            get_app, get_hotkey_running, get_hotkey_stop_tx, get_hotkey_store, get_main_window,
        },
        keyboard::{type_last_clipboard, type_last_clipboard_linux},
        window::{sync_clipboard_history_toggle, toggle_main_window},
    },
    types::types::Key,
    utils::{
        hotkey_manager::{register_hotkeys, unregister_hotkeys, upsert_hotkeys_in_store},
        tauri::config::HotkeyEvent,
    },
};
use core::time::Duration;
use global_hotkey::{GlobalHotKeyEvent, HotKeyState};
use regex::Regex;
use tauri::Emitter;
use tokio::sync::oneshot;

pub fn init_hotkey_listener() -> () {
    let receiver = GlobalHotKeyEvent::receiver();

    unregister_hotkeys(true);
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            upsert_hotkeys_in_store().await.unwrap();
        })
    });
    register_hotkeys(false);

    // If there's an existing sender, send a stop signal to the previous task
    if let Some(sender) = get_hotkey_stop_tx().take() {
        let _ = sender.send(());
    }

    let (new_stop_tx, mut stop_rx) = oneshot::channel();
    *get_hotkey_stop_tx() = Some(new_stop_tx);

    tauri::async_runtime::spawn(async move {
        loop {
            if let Ok(event) = receiver.try_recv() {
                if event.state == HotKeyState::Pressed {
                    printlog!("hotkey caught {:?}", event);
                    let hotkey = get_hotkey_store().get(&event.id).cloned();
                    if let Some(hotkey) = hotkey {
                        parse_hotkey_event(&hotkey).await;
                    }
                }
            }

            if stop_rx.try_recv().is_ok() {
                break;
            }

            std::thread::sleep(Duration::from_millis(10));
        }
    });
}

pub async fn parse_hotkey_event(key: &Key) {
    let event = key.event.parse::<HotkeyEvent>();

    printlog!("event: {:?}", event);

    match event {
        Ok(HotkeyEvent::WindowDisplayToggle) => toggle_main_window(),
        Ok(e @ HotkeyEvent::ScrollToTop) => {
            *get_hotkey_running() = true;
            get_main_window().emit(e.as_str(), ()).unwrap();
        }
        Ok(HotkeyEvent::TypeClipboard) => {
            if cfg!(target_os = "linux") {
                type_last_clipboard_linux().await.unwrap();
            } else {
                type_last_clipboard().await;
            }
        }
        Ok(HotkeyEvent::SyncClipboardHistory) => sync_clipboard_history_toggle().await,
        Ok(e @ (HotkeyEvent::Preferences | HotkeyEvent::About)) => get_main_window()
            .emit("open_window", Some(e.as_str()))
            .unwrap(),

        Ok(HotkeyEvent::Exit) => get_app().exit(1),
        Ok(
            e @ (HotkeyEvent::RecentClipboard
            | HotkeyEvent::StarredClipboard
            | HotkeyEvent::History
            | HotkeyEvent::ViewMore),
        ) => {
            *get_hotkey_running() = true;
            get_main_window()
                .emit("change_tab", Some(e.as_str()))
                .unwrap();
        }
        Ok(
            e @ (HotkeyEvent::Digit1
            | HotkeyEvent::Digit2
            | HotkeyEvent::Digit3
            | HotkeyEvent::Digit4
            | HotkeyEvent::Digit5
            | HotkeyEvent::Digit6
            | HotkeyEvent::Digit7
            | HotkeyEvent::Digit8
            | HotkeyEvent::Digit9
            | HotkeyEvent::Num1
            | HotkeyEvent::Num2
            | HotkeyEvent::Num3
            | HotkeyEvent::Num4
            | HotkeyEvent::Num5
            | HotkeyEvent::Num6
            | HotkeyEvent::Num7
            | HotkeyEvent::Num8
            | HotkeyEvent::Num9),
        ) => {
            let num = Regex::new(r"\d+")
                .unwrap()
                .find_iter(e.as_str())
                .map(|m| m.as_str())
                .collect::<String>()
                .parse::<u64>()
                .unwrap_or_default();
            let _ = copy_clipboard_from_index(num - 1).await;
        }
        Err(()) => panic!("Error parsing hotkey event"),
    };
}
