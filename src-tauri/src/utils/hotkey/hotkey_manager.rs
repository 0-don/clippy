use crate::{
    service::hotkey::get_all_hotkeys_db,
    types::types::Key,
    utils::setup::{GLOBAL_EVENTS, HOTKEYS, HOTKEY_MANAGER},
};
use global_hotkey::hotkey::HotKey;

pub fn register_hotkeys(all: bool) {
    println!("hotkey register");
    let hotkeys_store = HOTKEYS.get().unwrap().lock().unwrap();
    let hotkey_manager = HOTKEY_MANAGER.get().unwrap();

    for (_, hotkey) in hotkeys_store.iter() {
        if all || hotkey.global {
            if hotkey_manager.register(hotkey.hotkey.clone()).is_err() {
                hotkey_manager.unregister(hotkey.hotkey.clone()).unwrap();
                hotkey_manager.register(hotkey.hotkey.clone()).unwrap();
            }
        }
    }
}

pub fn unregister_hotkeys(all: bool) {
    println!("hotkey unregister");
    let hotkeys_store = HOTKEYS.get().unwrap().lock().unwrap();
    let hotkey_manager = HOTKEY_MANAGER.get().unwrap();

    for (_, hotkey) in hotkeys_store.iter() {
        if all || !hotkey.global {
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
