use super::tauri::config::{GLOBAL_EVENTS, HOTKEYS, HOTKEY_MANAGER};
use crate::{printlog, service::hotkey::get_all_hotkeys_db, types::types::Key};
use global_hotkey::hotkey::HotKey;
use global_hotkey::GlobalHotKeyManager;
use std::{collections::HashMap, sync::MutexGuard};

fn get_hotkeys_and_manager() -> (
    MutexGuard<'static, HashMap<u32, Key>>,
    &'static GlobalHotKeyManager,
) {
    let hotkeys_store = HOTKEYS
        .get()
        .expect("Failed to get HOTKEYS")
        .lock()
        .expect("Failed to lock HOTKEYS");
    let hotkey_manager = HOTKEY_MANAGER.get().expect("Failed to get HOTKEY_MANAGER");
    (hotkeys_store, hotkey_manager)
}

pub fn register_hotkeys(all: bool) {
    // Get the data we need from the locked resource as quickly as possible
    let (hotkeys_data, hotkey_manager) = get_hotkeys_and_manager();
    let hotkeys_data: Vec<_> = hotkeys_data.iter().collect();

    printlog!("register_hotkeys start {}", hotkeys_data.len());

    let mut hotkeys_to_register = Vec::new();
    let mut failed_hotkeys = Vec::new();

    // Collect the hotkeys we want to register
    for (_, hotkey) in hotkeys_data.iter() {
        if all || hotkey.global {
            hotkeys_to_register.push(hotkey.hotkey);
        }
    }

    // Use bulk registration
    if hotkey_manager.register_all(&hotkeys_to_register).is_err() {
        for &hotkey in hotkeys_to_register.iter() {
            if hotkey_manager.register(hotkey).is_err() {
                failed_hotkeys.push(hotkey);
            }
        }

        // Handle failed registrations: try unregistering and registering again
        for &hotkey in failed_hotkeys.iter() {
            let _ = hotkey_manager.unregister(hotkey);
            let _ = hotkey_manager.register(hotkey);
        }
    }

    printlog!("register_hotkeys end {}", hotkeys_to_register.len());
}

pub fn unregister_hotkeys(all: bool) {
    let (hotkeys_store, hotkey_manager) = get_hotkeys_and_manager();
    printlog!("unregister_hotkeys start {}", hotkeys_store.len());

    let mut hotkeys_to_unregister = Vec::new();

    // Collect the hotkeys we want to unregister
    for (_, hotkey) in hotkeys_store.iter() {
        if all || !hotkey.global {
            hotkeys_to_unregister.push(hotkey.hotkey);
        }
    }

    // Use bulk unregistration if available
    // If not available, use a loop similar to the registration function
    hotkey_manager
        .unregister_all(&hotkeys_to_unregister)
        .unwrap();

    printlog!("unregister_hotkeys end {}", hotkeys_to_unregister.len());
}

fn insert_hotkey_into_store(key: Key) {
    let mut hotkeys_lock = HOTKEYS.get().unwrap().lock().unwrap();

    if hotkeys_lock.get(&key.id).is_some() {
        hotkeys_lock.remove(&key.id).unwrap();
    }
    hotkeys_lock.insert(key.id, key);
}

pub async fn upsert_hotkeys_in_store() -> Result<(), Box<dyn std::error::Error>> {
    let hotkeys = get_all_hotkeys_db().await?;

    for hotkey in hotkeys {
        let hotkey_str = parse_shortcut(
            hotkey.ctrl,
            hotkey.alt,
            hotkey.shift,
            &format!("Key{}", &hotkey.key.to_uppercase()),
        );
        let key: HotKey = hotkey_str.parse()?;
        let key_struct = Key {
            id: key.id(),
            global: GLOBAL_EVENTS.contains(&hotkey.event.as_str()),
            event: hotkey.event,
            key_str: hotkey_str,
            ctrl: hotkey.ctrl,
            alt: hotkey.alt,
            shift: hotkey.shift,
            key: hotkey.key,
            hotkey: key,
        };
        insert_hotkey_into_store(key_struct);
    }

    // Add 1..9 regular keys which are not global
    for i in 1..=9 {
        let hotkey_digit = parse_shortcut(false, false, false, &format!("Digit{}", i));
        let key_digit: HotKey = hotkey_digit.parse()?;
        let hotkey_num = parse_shortcut(false, false, false, &format!("Numpad{}", i));
        let key_num: HotKey = hotkey_num.parse()?;

        let key_structs = vec![
            Key {
                id: key_digit.id(),
                global: false,
                event: format!("digit_{}", i),
                key_str: hotkey_digit.clone(),
                ctrl: false,
                alt: false,
                shift: false,
                key: i.to_string(),
                hotkey: key_digit,
            },
            Key {
                id: key_num.id(),
                global: false,
                event: format!("num_{}", i),
                key_str: hotkey_num,
                ctrl: false,
                alt: false,
                shift: false,
                key: i.to_string(),
                hotkey: key_num,
            },
        ];
        for key_struct in key_structs {
            insert_hotkey_into_store(key_struct);
        }
    }

    Ok(())
}

pub fn parse_shortcut(ctrl: bool, alt: bool, shift: bool, key: &str) -> String {
    let modifiers: Vec<&str> = vec![
        if ctrl { "Control" } else { "" },
        if alt { "Alt" } else { "" },
        if shift { "Shift" } else { "" },
    ]
    .into_iter()
    .filter(|s| !s.is_empty())
    .collect();

    format!(
        "{}{}{}",
        modifiers.join("+"),
        if !modifiers.is_empty() { "+" } else { "" },
        key.to_uppercase()
    )
}
