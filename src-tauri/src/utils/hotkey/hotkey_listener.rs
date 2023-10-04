use crate::{
    service::{
        clipboard::copy_clipboard_from_index, hotkey::get_all_hotkeys_db,
        window::toggle_main_window,
    },
    types::types::Key,
    utils::{
        clipboard::clipboard_helper::type_last_clipboard,
        setup::{HotkeyEvent, APP, GLOBAL_EVENTS, HOTKEYS, HOTKEY_MANAGER, HOTKEY_STOP_TX},
    },
};
use core::time::Duration;
use global_hotkey::hotkey::HotKey;
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
        Ok(HotkeyEvent::SyncClipboardHistory) => window.emit("sync_popup", Some(())).unwrap(),
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

pub fn register_hotkeys() {
    let hotkeys_store = HOTKEYS.get().unwrap().lock().unwrap();
    let hotkey_manager = HOTKEY_MANAGER.get().unwrap();
    let window = APP.get().unwrap().get_window("main").unwrap();

    for (_, hotkey) in hotkeys_store.iter() {
        if window.is_visible().unwrap() {
            hotkey_manager.register(hotkey.hotkey.clone()).unwrap();
        } else if hotkey.global {
            let key = hotkey_manager.register(hotkey.hotkey.clone());
            if key.is_err() {
                hotkey_manager.unregister(hotkey.hotkey.clone()).unwrap();
                hotkey_manager.register(hotkey.hotkey.clone()).unwrap();
            } else {
                key.unwrap();
            }
        }
    }
}

pub fn unregister_hotkeys(all: bool) {
    let hotkeys_store = HOTKEYS.get().unwrap().lock().unwrap();
    let hotkey_manager = HOTKEY_MANAGER.get().unwrap();

    for (_, hotkey) in hotkeys_store.iter() {
        if all {
            hotkey_manager.unregister(hotkey.hotkey.clone()).unwrap();
        } else if !hotkey.global {
            hotkey_manager.unregister(hotkey.hotkey.clone()).unwrap();
        }
    }
}

pub async fn upsert_hotkeys_in_store() -> anyhow::Result<()> {
    let hotkeys = get_all_hotkeys_db().await?;
    let mut hotkey_store = HOTKEYS.get().unwrap().lock().unwrap();

    for hotkey in hotkeys {
        let hotkey_str: String = parse_shortcut(
            hotkey.ctrl,
            hotkey.alt,
            hotkey.shift,
            &hotkey.key.to_lowercase(),
        );

        let key: HotKey = hotkey_str.parse()?;

        let global = GLOBAL_EVENTS.contains(&hotkey.event.as_str());

        let key = Key {
            id: key.id(),
            global,
            event: hotkey.event,
            key_str: hotkey_str,
            ctrl: hotkey.ctrl,
            alt: hotkey.alt,
            shift: hotkey.shift,
            key: hotkey.key,
            hotkey: key,
        };

        if hotkey_store.get(&key.id).is_some() {
            let _ = hotkey_store.remove(&key.id);
        }

        hotkey_store.insert(key.id, key);
    }

    // Add 1..9 regular keys which are not global
    for i in 1..=9 {
        let hotkey_str = parse_shortcut(false, false, false, &i.to_string());
        let key: HotKey = hotkey_str.parse()?;

        let key_struct = Key {
            id: key.id(),
            global: false,               // These keys are not global
            event: format!("key_{}", i), // Adjust this if you have specific events for the keys
            key_str: hotkey_str,
            ctrl: false,
            alt: false,
            shift: false,
            key: i.to_string(),
            hotkey: key,
        };

        if hotkey_store.get(&key_struct.id).is_some() {
            let _ = hotkey_store.remove(&key_struct.id);
        }

        hotkey_store.insert(key_struct.id, key_struct);
    }

    Ok(())
}

pub fn parse_shortcut(ctrl: bool, alt: bool, shift: bool, key: &str) -> String {
    let mut modifiers = Vec::new();
    if ctrl {
        modifiers.push("Control");
    }
    if alt {
        modifiers.push("Alt");
    }
    if shift {
        modifiers.push("Shift");
    }

    format!(
        "{}{}Key{}",
        modifiers.join("+"),
        if !modifiers.is_empty() { "+" } else { "" },
        key.to_uppercase()
    )
}
