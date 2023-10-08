use crate::{
    printlog,
    service::{
        clipboard::copy_clipboard_from_index,
        window::{sync_clipboard_history, toggle_main_window},
    },
    types::types::Key,
    utils::{
        clipboard_manager::type_last_clipboard,
        hotkey_manager::{register_hotkeys, unregister_hotkeys, upsert_hotkeys_in_store},
        tauri::config::{HotkeyEvent, APP, HOTKEYS, HOTKEY_STOP_TX, MAIN_WINDOW},
    },
};
use core::time::Duration;
use global_hotkey::GlobalHotKeyEvent;
use tauri::regex::Regex;
use tauri::Manager;
use tokio::sync::oneshot;

pub fn init_hotkey_listener(all: bool) -> () {
    let receiver = GlobalHotKeyEvent::receiver();

    tauri::async_runtime::spawn(async move {
        unregister_hotkeys(true);
        let _ = upsert_hotkeys_in_store().await;
        register_hotkeys(all)
    });

    // If there's an existing sender, send a stop signal to the previous task
    if let Some(sender) = HOTKEY_STOP_TX.get().unwrap().lock().unwrap().take() {
        let _ = sender.send(());
    }

    let (new_stop_tx, mut stop_rx) = oneshot::channel();
    *HOTKEY_STOP_TX.get().unwrap().lock().unwrap() = Some(new_stop_tx);

    tauri::async_runtime::spawn(async move {
        loop {
            if let Ok(event) = receiver.try_recv() {
                printlog!("hotkey caught");
                let hotkey = {
                    let hotkeys = HOTKEYS.get().unwrap().lock().unwrap();
                    hotkeys.get(&event.id).cloned()
                };
                if let Some(hotkey) = hotkey {
                    parse_hotkey_event(&hotkey).await;
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

    let window = MAIN_WINDOW
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .get_window("main")
        .unwrap();

    printlog!("event: {:?}", event);

    match event {
        Ok(HotkeyEvent::WindowDisplayToggle) => toggle_main_window(),
        Ok(HotkeyEvent::TypeClipboard) => type_last_clipboard().await,
        Ok(HotkeyEvent::SyncClipboardHistory) => sync_clipboard_history().await.unwrap(),
        Ok(e @ HotkeyEvent::Preferences) => window.emit("open_window", Some(e.as_str())).unwrap(),
        Ok(e @ HotkeyEvent::About) => window.emit("open_window", Some(e.as_str())).unwrap(),
        Ok(HotkeyEvent::Exit) => APP.get().unwrap().exit(1),
        Ok(
            e @ (HotkeyEvent::RecentClipboard
            | HotkeyEvent::StarredClipboard
            | HotkeyEvent::History
            | HotkeyEvent::ViewMore),
        ) => {
            printlog!("change_tab: {:?}", e);
            window.emit("change_tab", Some(e.as_str())).unwrap();
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
        Err(()) => println!("Error parsing hotkey event"),
    };
}
