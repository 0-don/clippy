use crate::service::window::toggle_main_window;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_positioner::on_tray_event;

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
        } => toggle_main_window(),
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => toggle_main_window(),
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {}
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "open" => toggle_main_window(),
            "quit" => app.exit(1),
            _ => panic!("Unhandled tray event"),
        },
        _ => panic!("Unhandled tray event"),
    }
}
