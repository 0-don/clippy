use super::tauri::config::GLOBAL_EVENTS;
use crate::{
    printlog,
    service::{
        global::{get_hotkey_manager, get_hotkey_store},
        hotkey::get_all_hotkeys_db,
    },
    types::types::Key,
};
use global_hotkey::hotkey::HotKey;
#[cfg(target_os = "windows")]
use crate::service::global::get_app;

pub fn register_hotkeys(all: bool) {
    #[cfg(target_os = "windows")]
    {
        get_app()
            .run_on_main_thread(move || {
                register_hotkeys_inner(all);
            })
            .unwrap();
    }
    #[cfg(not(target_os = "windows"))]
    {
        register_hotkeys_inner(all);
    }
}

pub fn unregister_hotkeys(all: bool) {
    #[cfg(target_os = "windows")]
    {
        get_app()
            .run_on_main_thread(move || {
                unregister_hotkeys_inner(all);
            })
            .unwrap();
    }
    #[cfg(not(target_os = "windows"))]
    {
        unregister_hotkeys_inner(all);
    }
}

fn register_hotkeys_inner(all: bool) {
    for (_, hotkey) in get_hotkey_store().iter_mut() {
        if !hotkey.state && (all || hotkey.is_global) {
            let key = hotkey.hotkey.clone();
            match get_hotkey_manager().register(key) {
                Ok(_) => {
                    printlog!("register_hotkeys {:?} {:?}", hotkey.event, hotkey.key_str);
                }
                Err(e) => {
                    printlog!(
                        "register_hotkeys error {:?} {:?} {:?}",
                        e,
                        hotkey.event,
                        hotkey.key_str
                    );
                }
            };
            hotkey.state = true;
        }
    }
}

fn unregister_hotkeys_inner(all: bool) {
    for (_, hotkey) in get_hotkey_store().iter_mut() {
        if hotkey.state && (all || !hotkey.is_global) {
            let key = hotkey.hotkey.clone();
            match get_hotkey_manager().unregister(key) {
                Ok(_) => {
                    printlog!("unregister_hotkeys {:?} {:?}", hotkey.event, hotkey.key_str);
                }
                Err(e) => {
                    printlog!(
                        "unregister_hotkeys error {:?} {:?} {:?}",
                        e,
                        hotkey.event,
                        hotkey.key_str
                    );
                }
            };
            hotkey.state = false;
        }
    }
}

fn insert_hotkey_into_store(key: Key) {
    let mut hotkeys_lock = get_hotkey_store();

    if hotkeys_lock.get(&key.id).is_some() {
        hotkeys_lock.remove(&key.id).unwrap();
    }
    hotkeys_lock.insert(key.id, key);
}

pub async fn upsert_hotkeys_in_store() -> Result<(), Box<dyn std::error::Error>> {
    get_hotkey_store().clear();
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
    if !cfg!(target_os = "linux") {
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
                    key_str: hotkey_digit.clone(),
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
                insert_hotkey_into_store(key_struct);
            }
        }
    }

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
