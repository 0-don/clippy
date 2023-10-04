use crate::{
    service::{
        clipboard::copy_clipboard_from_index,
        window::{sync_clipboard_history, toggle_main_window},
    },
    types::types::Key,
    utils::{
        clipboard::clipboard_helper::type_last_clipboard,
        hotkey::hotkey_manager::{register_hotkeys, unregister_hotkeys, upsert_hotkeys_in_store},
        setup::{HotkeyEvent, APP, HOTKEYS, HOTKEY_STOP_TX},
    },
};
use core::time::Duration;
use global_hotkey::GlobalHotKeyEvent;
use tauri::regex::Regex;
use tauri::Manager;
use tokio::sync::oneshot;

pub fn init_hotkey_listener() -> () {
    let receiver = GlobalHotKeyEvent::receiver();
    println!("init_hotkey_listener");

    tauri::async_runtime::spawn(async {
        unregister_hotkeys(true);
        let _ = upsert_hotkeys_in_store().await;
        register_hotkeys()
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

            std::thread::sleep(Duration::from_millis(100));
        }
    });
}

pub async fn parse_hotkey_event(key: &Key) {
    let event = key.event.parse::<HotkeyEvent>();

    let window = APP.get().unwrap().get_window("main").unwrap();

    match event {
        Ok(HotkeyEvent::WindowDisplayToggle) => toggle_main_window(),
        Ok(HotkeyEvent::TypeClipboard) => type_last_clipboard().await,
        Ok(HotkeyEvent::SyncClipboardHistory) => sync_clipboard_history(),
        Ok(HotkeyEvent::Preferences) => window.emit("open_preferences_window", Some(())).unwrap(),
        Ok(HotkeyEvent::About) => window.emit("open_about_window", Some(())).unwrap(),
        Ok(HotkeyEvent::Exit) => APP.get().unwrap().exit(1),
        Ok(
            e @ (HotkeyEvent::RecentClipboard
            | HotkeyEvent::StarredClipboard
            | HotkeyEvent::History
            | HotkeyEvent::ViewMore),
        ) => window.emit("change_tab", Some(e.as_str())).unwrap(),
        Ok(
            e @ (HotkeyEvent::Key1
            | HotkeyEvent::Key2
            | HotkeyEvent::Key3
            | HotkeyEvent::Key4
            | HotkeyEvent::Key5
            | HotkeyEvent::Key6
            | HotkeyEvent::Key7
            | HotkeyEvent::Key8
            | HotkeyEvent::Key9),
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
    }
}
