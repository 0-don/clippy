use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_positioner::{on_tray_event, Position, WindowExt};

use crate::service::window::init_hotkey;

pub fn system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    system_tray
}

pub fn system_tray_event(app: &tauri::AppHandle, event: SystemTrayEvent) {
    on_tray_event(app, &event);
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let win = app.get_window("main").unwrap();
            let _ = win.move_window(Position::TrayCenter);
            init_hotkey();
            let _ = win.show();
            let _ = win.set_focus();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => app.exit(1),
            _ => {}
        },
        _ => {}
    }
}
