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
    // let res = tokio::runtime::Runtime::new().unwrap().spawn(async move {
    //     println!("text: {:?}", 1);
    //     let model = parse_model();
    //     println!("text: {:?}", 2);
    //     let res = insert(model).await;
    //     println!("text: {:?}", 3);
    // });

    let _ = tauri::async_runtime::spawn(async move {
        let master = Master::new(Handler).run();
    });

    Ok(())
}
