use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};

use crate::utils::setup::APP;

#[tauri::command]
pub fn window_on_mouse() {
    let win = APP.get().unwrap().get_window("main").unwrap();

    if win.is_visible().unwrap() {
        let _ = win.hide();
    } else {
        let _ = win.show();
        let _ = win.set_focus();
    }

    // let enigo = Enigo::new();
    // let (x, y) = enigo.mouse_location();

    // let _ = win.set_position(PhysicalPosition::new(x, y));

    let _ = win.move_window(Position::BottomRight);
}

#[tauri::command]
pub fn is_production() -> bool {
    let state = if cfg!(debug_assertions) { false } else { true };
    state
}
