use crate::{
    types::types::{Config, DataPath},
    utils::tauri::config::{APP, MAIN_WINDOW, WINDOW_STOP_TX},
};
use std::{
    fs::{self, read_to_string},
    path::{Path, PathBuf},
};
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri_plugin_positioner::{Position, WindowExt};

pub fn toggle_main_window() {
    *WINDOW_STOP_TX.get().unwrap().lock().unwrap() = None;
    let window = MAIN_WINDOW.get().unwrap().lock().unwrap();

    if window.is_visible().unwrap() {
        window.hide().unwrap();
    } else {
        window.move_window(Position::BottomRight).unwrap();
        window.show().unwrap();
    }
}

pub fn get_data_path() -> DataPath {
    let config_path = APP
        .get()
        .unwrap()
        .path_resolver()
        .app_data_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let _ = fs::create_dir_all(&config_path);

    // let config_file = Path::new(&config_dir).join("config.json");
    let config_file_path = [&config_path, "config.json"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string();

    let db_file_path = [&config_path, "clippy.sqlite"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string();

    DataPath {
        config_path,
        db_file_path,
        config_file_path,
    }
}

pub async fn sync_clipboard_history() -> Result<(), ()> {
    let data_path = get_data_path();

    // get local config from app data
    let mut config: Config =
        serde_json::from_str(&read_to_string(&data_path.config_file_path).unwrap()).unwrap();
    let dir = FileDialogBuilder::new().pick_folder();

    // check if user disabled backup or not
    if dir.is_some() {
        // path to backup file
        let dir_file = dir.unwrap().to_string_lossy().to_string();

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

    Ok(())
}
