use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_positioner::{on_tray_event, Position, WindowExt};

use crate::service::window::init_hotkey;

pub fn system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let open: CustomMenuItem = CustomMenuItem::new("open".to_string(), "Open");
    let tray_menu = SystemTrayMenu::new().add_item(open).add_item(quit);
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
            println!("Clicked on tray icon");
            let win = app.get_window("main").unwrap();
            let _ = win.move_window(Position::TrayCenter);
            init_hotkey();
            let _ = win.show();
            let _ = win.set_focus();
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "open" => {
                let win = app.get_window("main").unwrap();
                let _ = win.move_window(Position::BottomRight);
                init_hotkey();
                let _ = win.show();
                let _ = win.set_focus();
            }
            "quit" => app.exit(1),
            _ => {}
        },
        _ => {}
    }
}
