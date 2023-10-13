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
    let (mut hotkeys_store, hotkey_manager) = get_hotkeys_and_manager();
    printlog!("register_hotkeys start");

    let mut instant_hotkeys = Vec::new();
    let mut delayed_hotkeys = Vec::new();

    // Collect the hotkeys we want to register
    for (_, hotkey) in hotkeys_store.iter() {
        if all || hotkey.is_global {
            if hotkey.event.contains("digit_") || hotkey.event.contains("num_") {
                delayed_hotkeys.push(hotkey.hotkey);
            } else {
                instant_hotkeys.push(hotkey.hotkey);
            }
        }
    }

    for hotkey in instant_hotkeys {
        if let Some(hotkey) = hotkeys_store.get_mut(&hotkey.id()) {
            if !hotkey.state {
                let _ = hotkey_manager.register(hotkey.hotkey);
                hotkey.state = true;
            }
        }
    }

    if cfg!(target_os = "linux") {
        tauri::async_runtime::spawn(async {
            let mut hotkeys_store = HOTKEYS
                .get()
                .expect("Failed to get HOTKEYS")
                .lock()
                .expect("Failed to lock HOTKEYS");
            for hotkey in delayed_hotkeys {
                if let Some(hotkey) = hotkeys_store.get_mut(&hotkey.id()) {
                    printlog!("registering hotkey: {:?}", hotkey);
                    if !hotkey.state {
                        let _ = hotkey_manager.register(hotkey.hotkey);
                        hotkey.state = true;
                    }
                }
            }
            printlog!("register_hotkeys end");
        });
    } else {
        for hotkey in delayed_hotkeys {
            if let Some(hotkey) = hotkeys_store.get_mut(&hotkey.id()) {
                printlog!("registering hotkey: {:?}", hotkey);
                if !hotkey.state {
                    let _ = hotkey_manager.register(hotkey.hotkey);
                    hotkey.state = true;
                }
            }
        }
        printlog!("register_hotkeys end");
    }
}

pub fn unregister_hotkeys(all: bool) {
    printlog!("unregister_hotkeys start");
    let (mut hotkeys_store, hotkey_manager) = get_hotkeys_and_manager();

    let mut hotkeys_to_unregister = Vec::new();

    // Collect the hotkeys we want to unregister
    for (_, hotkey) in hotkeys_store.iter() {
        if all || !hotkey.is_global {
            hotkeys_to_unregister.push(hotkey.hotkey);
        }
    }

    if cfg!(target_os = "linux") {
        tauri::async_runtime::spawn(async move {
            let mut hotkeys_store = HOTKEYS
                .get()
                .expect("Failed to get HOTKEYS")
                .lock()
                .expect("Failed to lock HOTKEYS");
            for hotkey in hotkeys_to_unregister {
                if let Some(hotkey) = hotkeys_store.get_mut(&hotkey.id()) {
                    if hotkey.state {
                        hotkey_manager.unregister(hotkey.hotkey).unwrap();
                        hotkey.state = false;
                    }
                }
            }
        });
    } else {
        for hotkey in hotkeys_to_unregister {
            if let Some(hotkey) = hotkeys_store.get_mut(&hotkey.id()) {
                if hotkey.state {
                    hotkey_manager.unregister(hotkey.hotkey).unwrap();
                    hotkey.state = false;
                }
            }
        }
    }

    printlog!("unregister_hotkeys end");
}

pub fn unregister_hotkeys_async(all: bool) {
    tauri::async_runtime::spawn(async move { unregister_hotkeys(all) });
}

fn insert_hotkey_into_store(key: Key) {
    let mut hotkeys_lock = HOTKEYS.get().unwrap().lock().unwrap();

    if hotkeys_lock.get(&key.id).is_some() {
        hotkeys_lock.remove(&key.id).unwrap();
    }
    hotkeys_lock.insert(key.id, key);
}

pub async fn upsert_hotkeys_in_store() -> Result<(), Box<dyn std::error::Error>> {
    HOTKEYS.get().unwrap().lock().unwrap().clear();
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
            state: false,
            is_global: GLOBAL_EVENTS.contains(&hotkey.event.as_str()),
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
    // for i in 1..=9 {
    //     let hotkey_digit = parse_shortcut(false, false, false, &format!("Digit{}", i));
    //     let key_digit: HotKey = hotkey_digit.parse()?;
    //     let hotkey_num = parse_shortcut(false, false, false, &format!("Numpad{}", i));
    //     let key_num: HotKey = hotkey_num.parse()?;

    //     let key_structs = vec![
    //         Key {
    //             id: key_digit.id(),
    //             global: false,
    //             event: format!("digit_{}", i),
    //             key_str: hotkey_digit.clone(),
    //             ctrl: false,
    //             alt: false,
    //             shift: false,
    //             key: i.to_string(),
    //             hotkey: key_digit,
    //         },
    //         Key {
    //             id: key_num.id(),
    //             global: false,
    //             event: format!("num_{}", i),
    //             key_str: hotkey_num,
    //             ctrl: false,
    //             alt: false,
    //             shift: false,
    //             key: i.to_string(),
    //             hotkey: key_num,
    //         },
    //     ];
    //     for key_struct in key_structs {
    //         insert_hotkey_into_store(key_struct);
    //     }
    // }

    Ok(())
}

pub fn parse_shortcut(ctrl: bool, alt: bool, shift: bool, key: &str) -> String {
    let mut modifiers = String::new();
    if ctrl {
        modifiers += "Control+";
    }
    if alt {
        modifiers += "Alt+";
    }
    if shift {
        modifiers += "Shift+";
    }
    format!("{}{}", modifiers, key.to_uppercase())
}
