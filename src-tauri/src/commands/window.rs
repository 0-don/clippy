use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};

use crate::{service::window::init_hotkey, utils::setup::APP};

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

    let _ = win.move_window(Position::BottomRight);
}

// https://docs.rs/dirs_next/
#[tauri::command]
pub fn sync_clipboard_history() {
    // println!(
    //     "{:?}",

    // );
}
