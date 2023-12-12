use super::tauri::config::GLOBAL_EVENTS;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::service::global::get_app;
use crate::{
    printlog,
    service::{
        global::{get_hotkey_manager, get_hotkey_store},
        hotkey::get_all_hotkeys_db,
    },
    types::types::Key,
};
use global_hotkey::hotkey::HotKey;

pub fn register_hotkeys(all: bool) {
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        get_app()
            .run_on_main_thread(move || {
                register_hotkeys_inner(all);
            })
            .unwrap();
    }
    #[cfg(target_os = "linux")]
    {
        register_hotkeys_inner(all);
    }
}

pub fn unregister_hotkeys(all: bool) {
    #[cfg(any(target_os = "windows", target_os = "macos"))]
    {
        get_app()
            .run_on_main_thread(move || {
                unregister_hotkeys_inner(all);
            })
            .unwrap();
    }
    #[cfg(target_os = "linux")]
    {
        unregister_hotkeys_inner(all);
    }
}

fn register_hotkeys_inner(all: bool) {
    let mut hotkeys_to_register = Vec::new();
    for (_, hotkey) in get_hotkey_store().iter_mut() {
        if !hotkey.state && (all || hotkey.is_global) {
            hotkeys_to_register.push(hotkey.hotkey.clone());
            hotkey.state = true;
        }
    }

    if let Err(e) = get_hotkey_manager().register_all(&hotkeys_to_register) {
        printlog!("register_hotkeys error: {:?}", e);
    }
}

fn unregister_hotkeys_inner(all: bool) {
    let mut hotkeys_to_unregister = Vec::new();
    for (_, hotkey) in get_hotkey_store().iter_mut() {
        if hotkey.state && (all || !hotkey.is_global) {
            hotkeys_to_unregister.push(hotkey.hotkey.clone());
            hotkey.state = false;
        }
    }

    if let Err(e) = get_hotkey_manager().unregister_all(&hotkeys_to_unregister) {
        printlog!("unregister_hotkeys error: {:?}", e);
    }
}

// fn register_hotkeys_inner(all: bool) {
//     for (_, hotkey) in get_hotkey_store().iter_mut() {
//         if !hotkey.state && (all || hotkey.is_global) {
//             let key = hotkey.hotkey.clone();
//             match get_hotkey_manager().register(key) {
//                 Ok(_) => {
//                     printlog!("register_hotkeys {:?} {:?}", hotkey.event, hotkey.key_str);
//                 }
//                 Err(e) => {
//                     printlog!(
//                         "register_hotkeys error {:?} {:?} {:?}",
//                         e,
//                         hotkey.event,
//                         hotkey.key_str
//                     );
//                 }
//             };
//             hotkey.state = true;
//         }
//     }
// }

// fn unregister_hotkeys_inner(all: bool) {
//     for (_, hotkey) in get_hotkey_store().iter_mut() {
//         if hotkey.state && (all || !hotkey.is_global) {
//             let key = hotkey.hotkey.clone();
//             match get_hotkey_manager().unregister(key) {
//                 Ok(_) => {
//                     printlog!("unregister_hotkeys {:?} {:?}", hotkey.event, hotkey.key_str);
//                 }
//                 Err(e) => {
//                     printlog!(
//                         "unregister_hotkeys error {:?} {:?} {:?}",
//                         e,
//                         hotkey.event,
//                         hotkey.key_str
//                     );
//                 }
//             };
//             hotkey.state = false;
//         }
//     }
// }

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
            &format_key_for_parsing(&hotkey.key.to_uppercase()),
        );

        let key: HotKey = hotkey_str.parse()?;

        let key_struct = Key {
            id: key.id(),
            state: false,
            is_global: GLOBAL_EVENTS.contains(&hotkey.event.as_str()),
            event: hotkey.event,
            key_str: hotkey_str.clone(),
            ctrl: hotkey.ctrl,
            alt: hotkey.alt,
            shift: hotkey.shift,
            key: hotkey.key,
            hotkey: key,
        };

        insert_hotkey_into_store(key_struct);
    }

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
