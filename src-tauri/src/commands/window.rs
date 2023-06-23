use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};

use crate::{utils::setup::APP, service::window::init_hotkey};

#[tauri::command]
pub fn window_display_toggle() {
    let win = APP.get().unwrap().get_window("main").unwrap();

    if win.is_visible().unwrap() {
        let _ = win.hide();
    } else {
        init_hotkey();
        let _ = win.show();
        let _ = win.set_focus();
    }

    // let enigo = Enigo::new();
    // let (x, y) = enigo.mouse_location();

    // let _ = win.set_position(PhysicalPosition::new(x, y));

    let _ = win.move_window(Position::BottomRight);
}
