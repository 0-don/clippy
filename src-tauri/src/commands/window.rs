use crate::{
    service::{
        clipboard::count_clipboards_db,
        window::{get_data_path, open_window, sync_clipboard_history_toggle, toggle_main_window},
    },
    types::types::{Config, DatabaseInfo, WindowName},
};
use std::fs::{self, read_to_string};
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

#[tauri::command]
pub fn window_display_toggle() {
    toggle_main_window();
}

#[tauri::command]
pub fn open_new_window(window_name: WindowName) {
    open_window(window_name);
}

#[tauri::command]
pub fn exit_app() {
    std::process::exit(0);
}

#[tauri::command]
pub fn get_app_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}

#[tauri::command]
pub fn open_browser_url(url: String, app: AppHandle) {
    app.shell()
        .open(url, None)
        .map_err(|e| e.to_string())
        .expect("failed to open browser");
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

#[tauri::command]
pub async fn sync_clipboard_history() -> Result<(), ()> {
    sync_clipboard_history_toggle().await;

    Ok(())
}
