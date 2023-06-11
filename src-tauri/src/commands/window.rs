use clipboard_master::Master;
use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};

use crate::utils::{clipboard::Handler, setup::APP};

#[tauri::command]
pub async fn window_on_mouse() -> Result<(), String> {
    let win = APP.get().unwrap().get_window("main").unwrap();
    // let enigo = Enigo::new();
    // let (x, y) = enigo.mouse_location();

    // let _ = win.set_position(PhysicalPosition::new(x, y));

    let _ = win.move_window(Position::BottomRight);
    Ok(())
}

#[tauri::command]
pub async fn is_production() -> Result<bool, String> {
    let state = if cfg!(debug_assertions) { false } else { true };
    Ok(state)
}

#[tauri::command]
pub async fn init_listener() -> Result<(), ()> {
    tauri::async_runtime::spawn(async { Master::new(Handler).run() });
    // let _ = Master::new(Handler).run();

    Ok(())
}
