use crate::{
    service::window::{get_data_path, init_hotkey},
    types::types::Config,
    utils::setup::APP,
};
use std::{
    fs::{self, read_to_string},
    path::{Path, PathBuf},
};
use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};

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

#[tauri::command]
pub async fn get_db_size() -> Result<u64, ()> {
    let data_path = get_data_path();

    let config: Config =
        serde_json::from_str(&read_to_string(&data_path.config_file_path).unwrap()).unwrap();
    let size = fs::metadata(config.db).unwrap().len();

    Ok(size)
}

#[tauri::command]
pub async fn get_db_path() -> Result<String, ()> {
    let data_path = get_data_path();

    let config: Config =
        serde_json::from_str(&read_to_string(&data_path.config_file_path).unwrap()).unwrap();

    Ok(config.db)
}

#[tauri::command]
pub fn sync_clipboard_history(dir: Option<String>) {
    let data_path = get_data_path();

    // get local config from app data
    let mut config: Config =
        serde_json::from_str(&read_to_string(&data_path.config_file_path).unwrap()).unwrap();

    // check if user disabled backup or not
    if dir.is_some() {
        // path to backup file
        let dir_file = [&dir.unwrap(), "clippy.sqlite"]
            .iter()
            .collect::<PathBuf>()
            .to_string_lossy()
            .to_string();

        // check if backup file exists
        if !Path::new(&dir_file).exists() {
            // copy current database to backup location
            let _ = fs::copy(&config.db, &dir_file);
        }

        // overwrite config database location
        config.db = dir_file.to_string();

        // overwrite config file
        let _ = fs::write(
            &data_path.config_file_path,
            serde_json::to_string(&config).unwrap(),
        );
    } else {
        // copy backup file to default database location
        let _ = fs::copy(&config.db, &data_path.db_file_path);

        // overwrite config database default location
        config.db = data_path.db_file_path;

        // overwrite config file
        let _ = fs::write(
            &data_path.config_file_path,
            serde_json::to_string(&config).unwrap(),
        );
    }
}
