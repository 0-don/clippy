use crate::{
    service::{hotkey::get_all_hotkeys_db, window::toggle_main_window},
    types::types::Key,
    utils::setup::{HOTKEYS, HOTKEY_MANAGER, HOTKEY_STOP_TX},
};
use core::time::Duration;
use global_hotkey::hotkey::HotKey;
use global_hotkey::GlobalHotKeyEvent;
use tokio::sync::oneshot;

pub fn init_hotkey_listener() -> () {
    println!("init_hotkey_listener");

    // If there's an existing sender, send a stop signal to the previous task
    if let Some(sender) = HOTKEY_STOP_TX.get().unwrap().lock().unwrap().take() {
        let _ = sender.send(());
    }


    // let (new_stop_tx, mut stop_rx) = oneshot::channel();
    // *HOTKEY_STOP_TX.get().unwrap().lock().unwrap() = Some(new_stop_tx);
    let receiver = GlobalHotKeyEvent::receiver();
    tauri::async_runtime::spawn(async move {
        loop {
            if let Ok(event) = receiver.try_recv() {
                let hotkeys = HOTKEYS.get().unwrap().lock().unwrap();
                println!("Hotkey Pressed: {:?}", event.id);
                if let Some(hotkey) = hotkeys.get(&event.id) {
                    println!("Hotkey Pressed: {:?}", hotkey);
                    toggle_main_window();
                }
            }
            println!("looping");

            // if stop_rx.try_recv().is_ok() {
            //     break;
            // }
            std::thread::sleep(Duration::from_millis(1000));
        }
    });
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

        let key = Key {
            id: key.id(),
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

    println!("finsihed");

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
