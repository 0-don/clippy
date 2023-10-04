use crate::{
    service::{
        clipboard::count_clipboards_db,
        window::{get_data_path, init_hotkey},
    },
    types::types::{Config, DatabaseInfo},
    utils::setup::APP,
};
use std::fs::{self, read_to_string};
use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};

#[tauri::command]
pub fn window_display_toggle() {
    let win = APP.get().unwrap().get_window("main").unwrap();

    println!("Window is visible: {:?}", win.is_visible());

    if win.is_visible().unwrap() {
        println!("Hiding window");
        let _ = win.hide();
    } else {
        println!("Showing window");
        init_hotkey();
        let _ = win.show();
        let _ = win.set_focus();
    }

    let _ = win.move_window(Position::BottomRight);
}

#[tauri::command]
pub async fn get_db_size() -> Result<DatabaseInfo, ()> {
    let data_path = get_data_path();

    let config: Config =
        serde_json::from_str(&read_to_string(&data_path.config_file_path).unwrap()).unwrap();
    let size = fs::metadata(config.db).unwrap().len();

    let records = count_clipboards_db().await.unwrap();

    Ok(DatabaseInfo { records, size })
}

#[tauri::command]
pub async fn get_db_path() -> Result<String, ()> {
    let data_path = get_data_path();

    let config: Config =
        serde_json::from_str(&read_to_string(&data_path.config_file_path).unwrap()).unwrap();

    Ok(config.db)
}
