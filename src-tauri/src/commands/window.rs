use crate::{
    service::{
        clipboard::count_clipboards_db,
        window::{get_data_path, toggle_main_window},
    },
    types::types::{Config, DatabaseInfo},
};
use std::fs::{self, read_to_string};

#[tauri::command]
pub fn window_display_toggle() {
    toggle_main_window(None);
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
