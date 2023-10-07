use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_positioner::on_tray_event;

use crate::{service::window::toggle_main_window, printlog};

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
        } => toggle_main_window(None),
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "open" => toggle_main_window(None),
            "quit" => app.exit(1),
            _ => printlog!("Unhandled tray event"),
        },
        _ => printlog!("Unhandled tray event"),
    }
}
