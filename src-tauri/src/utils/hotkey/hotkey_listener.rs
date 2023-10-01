use core::time::Duration;

use global_hotkey::hotkey::HotKey;
use global_hotkey::GlobalHotKeyEvent;

use crate::utils::setup::HOTKEY_MANAGER;

pub fn init_hotkey_listener() -> () {
    println!("init_hotkey_listener");

    let hotkey_manager = HOTKEY_MANAGER.get().unwrap();

    let hotkey_str = parse_shortcut(true, false, false, "y");
    let hotkey: HotKey = hotkey_str.parse().unwrap();

    let _ = hotkey_manager.register(hotkey).unwrap();

    let receiver = GlobalHotKeyEvent::receiver();
    std::thread::spawn(|| loop {
        if let Ok(event) = receiver.try_recv() {
            println!("tray event: {event:?}");
        }
        std::thread::sleep(Duration::from_millis(100));
    });
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
