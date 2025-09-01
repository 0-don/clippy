use crate::prelude::*;
use crate::service::hotkey::get_all_hotkeys_db;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::tao::global::get_app;
use crate::tao::global::{
    get_global_hotkey_manager, get_global_hotkey_store, get_window_hotkey_manager,
    get_window_hotkey_store,
};
use common::{
    constants::GLOBAL_EVENTS,
    types::{hotkey::SafeHotKeyManager, types::Key},
};
use global_hotkey::{hotkey::HotKey, GlobalHotKeyManager};

pub fn register_hotkeys(global_only: bool) {
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        get_app()
            .run_on_main_thread(move || {
                register_hotkeys_inner(global_only);
            })
            .expect("Failed to register hotkeys");
    }
    #[cfg(target_os = "linux")]
    {
        register_hotkeys_inner(global_only);
    }
}

pub fn unregister_hotkeys(global_only: bool) {
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        get_app()
            .run_on_main_thread(move || {
                unregister_hotkeys_inner(global_only);
            })
            .expect("Failed to unregister hotkeys");
    }
    #[cfg(target_os = "linux")]
    {
        unregister_hotkeys_inner(global_only);
    }
}

fn register_hotkeys_inner(global_only: bool) {
    // Always register global hotkeys
    {
        let mut global_hotkeys_to_register = Vec::new();
        for (_, hotkey) in get_global_hotkey_store().iter_mut() {
            if !hotkey.state {
                global_hotkeys_to_register.push(hotkey.hotkey);
                hotkey.state = true;
            }
        }

        if !global_hotkeys_to_register.is_empty() {
            if let Err(e) = get_global_hotkey_manager().register_all(&global_hotkeys_to_register) {
                printlog!("register_global_hotkeys error: {:?}", e);
            }
        }
    }

    // Register window hotkeys only if not global_only
    if !global_only {
        let window_hotkeys_to_register: Vec<HotKey> = get_window_hotkey_store()
            .values()
            .map(|k| k.hotkey)
            .collect();

        if !window_hotkeys_to_register.is_empty() {
            // Create new window manager instance
            {
                let mut manager = get_window_hotkey_manager();
                *manager = Some(SafeHotKeyManager::new(
                    GlobalHotKeyManager::new().expect("Failed to create window hotkey manager"),
                ));
            }

            let manager = get_window_hotkey_manager();
            if let Some(ref manager) = *manager {
                if let Err(e) = manager.register_all(&window_hotkeys_to_register) {
                    printlog!("register_window_hotkeys error: {:?}", e);
                }
            }
        }
    }
}

fn unregister_hotkeys_inner(global_only: bool) {
    // Unregister global hotkeys only if explicitly requested (app shutdown)
    if global_only {
        let mut global_hotkeys_to_unregister = Vec::new();
        for (_, hotkey) in get_global_hotkey_store().iter_mut() {
            if hotkey.state {
                global_hotkeys_to_unregister.push(hotkey.hotkey);
                hotkey.state = false;
            }
        }

        if !global_hotkeys_to_unregister.is_empty() {
            if let Err(e) =
                get_global_hotkey_manager().unregister_all(&global_hotkeys_to_unregister)
            {
                printlog!("unregister_global_hotkeys error: {:?}", e);
            }
        }
    } else {
        // For window hotkeys, just drop the manager to force cleanup
        {
            let mut manager = get_window_hotkey_manager();
            *manager = None; // This will drop the manager and auto-unregister all its hotkeys
        }
    }
}

fn insert_global_hotkey_into_store(key: Key) {
    let mut hotkeys_lock = get_global_hotkey_store();

    if hotkeys_lock.get(&key.id).is_some() {
        hotkeys_lock
            .remove(&key.id)
            .expect("Failed to remove hotkey");
    }
    hotkeys_lock.insert(key.id, key);
}

fn insert_window_hotkey_into_store(key: Key) {
    let mut hotkeys_lock = get_window_hotkey_store();

    if hotkeys_lock.get(&key.id).is_some() {
        hotkeys_lock
            .remove(&key.id)
            .expect("Failed to remove hotkey");
    }
    hotkeys_lock.insert(key.id, key);
}

pub async fn upsert_hotkeys_in_store() -> Result<(), Box<dyn std::error::Error>> {
    // Clear both stores
    get_global_hotkey_store().clear();
    get_window_hotkey_store().clear();

    let hotkeys = get_all_hotkeys_db().await?;

    // Process database hotkeys
    for hotkey in hotkeys {
        let hotkey_str = parse_shortcut(
            hotkey.ctrl,
            hotkey.alt,
            hotkey.shift,
            &format_key_for_parsing(&hotkey.key.to_uppercase()),
        );

        let key: HotKey = hotkey_str.parse()?;

        let key_struct = Key {
            id: key.id(),
            state: false,
            is_global: GLOBAL_EVENTS.contains(&hotkey.event),
            event: hotkey.event.clone(),
            key_str: hotkey_str,
            ctrl: hotkey.ctrl,
            alt: hotkey.alt,
            shift: hotkey.shift,
            key: hotkey.key,
            hotkey: key,
        };

        // Insert into appropriate store based on whether it's a global event
        if GLOBAL_EVENTS.contains(&hotkey.event) {
            insert_global_hotkey_into_store(key_struct);
        } else {
            insert_window_hotkey_into_store(key_struct);
        }
    }

    // Add digit and numpad hotkeys (these are window-specific)
    for i in 1..=9 {
        let hotkey_digit = parse_shortcut(false, false, false, &format!("Digit{}", i));
        let key_digit: HotKey = hotkey_digit.parse()?;
        let hotkey_num = parse_shortcut(false, false, false, &format!("Numpad{}", i));
        let key_num: HotKey = hotkey_num.parse()?;

        let key_structs = vec![
            Key {
                id: key_digit.id(),
                state: false,
                is_global: false,
                event: format!("digit_{}", i),
                key_str: hotkey_digit,
                ctrl: false,
                alt: false,
                shift: false,
                key: i.to_string(),
                hotkey: key_digit,
            },
            Key {
                id: key_num.id(),
                state: false,
                is_global: false,
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
            insert_window_hotkey_into_store(key_struct);
        }
    }

    Ok(())
}

pub fn parse_shortcut(ctrl: bool, alt: bool, shift: bool, key: &str) -> String {
    let mut modifiers = String::new();
    if ctrl {
        modifiers += "CmdOrCtrl+";
    }
    if alt {
        modifiers += "Alt+";
    }
    if shift {
        modifiers += "Shift+";
    }
    let result = format!("{}{}", modifiers, key.to_uppercase());
    result
}

fn format_key_for_parsing(key: &str) -> String {
    if key.len() >= 2 && (key.starts_with('F') || key.starts_with('f')) {
        if let Ok(number) = key[1..].parse::<u32>() {
            if number >= 1 && number <= 24 {
                // Adjust the range if necessary
                return key.to_uppercase(); // This is a function key like F1, F2, ..., F24
            }
        }
    }

    match key.chars().next().unwrap_or_default() {
        '0'..='9' => format!("Digit{}", key), // For digits
        'A'..='Z' | 'a'..='z' => format!("Key{}", key.to_uppercase()), // For letters
        // Add additional cases here for other key types like F1-F12
        // ...
        _ => key.to_uppercase(), // Default case for other keys
    }
}
