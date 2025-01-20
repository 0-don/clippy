use crate::global::get_app;
use common::{
    constants::{CONFIG_NAME, DB_NAME},
    printlog,
    types::types::{Config, DataPath},
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

pub fn create_config() {
    let data_path = get_data_path();

    if Path::new(&data_path.config_file_path).exists() {
        return;
    }

    let config = Config {
        db: format!("{}", &data_path.db_file_path),
    };

    fs::write(
        &data_path.config_file_path,
        serde_json::to_string(&config).expect("Failed to serialize config"),
    )
    .expect("Failed to write config");

    printlog!(
        "config path {}",
        get_app()
            .path()
            .app_data_dir()
            .expect("Failed to get app data dir")
            .to_string_lossy()
            .to_string()
    );
}

pub fn get_data_path() -> DataPath {
    let config_path = if cfg!(debug_assertions) {
        // Get absolute project root directory
        let current_dir = std::env::current_dir()
            .expect("Failed to get current directory")
            .parent()
            .expect("Failed to get parent directory")
            .to_path_buf();

        current_dir.to_string_lossy().to_string()
    } else {
        // Use app data dir in production
        get_app()
            .path()
            .app_data_dir()
            .expect("Failed to get app data dir")
            .to_string_lossy()
            .to_string()
    };

    fs::create_dir_all(&config_path).expect("Failed to create config directory");

    let config_file_path = [&config_path, CONFIG_NAME]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string();

    let db_file_path = [&config_path, DB_NAME]
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

pub fn get_config() -> (Config, DataPath) {
    let data_path = get_data_path();

    let json = std::fs::read_to_string(&data_path.config_file_path).expect("Failed to read file");

    let config: Config = serde_json::from_str(&json).expect("Failed to parse JSON");

    (config, data_path)
}

pub fn change_clipboard_db_location_enable() {
    // get local config from app data
    let (mut config, data_path) = get_config();

    // Use blocking_pick_folder for synchronous folder selection
    if let Some(dir) = get_app().dialog().file().blocking_pick_folder() {
        // Convert path to string
        let dir = dir.to_string();
        let dir_file = format!("{}/clippy.sqlite", &dir);

        // check if backup file exists
        if !Path::new(&dir_file).exists() {
            // copy current database to backup location
            printlog!(
                "copying database to backup location {} {}",
                &config.db,
                &dir_file
            );
            fs::copy(&config.db, &dir_file).expect("Failed to copy database");
        }

        // overwrite config database location
        config.db = dir_file;

        // overwrite config file
        let _ = fs::write(
            &data_path.config_file_path,
            serde_json::to_string(&config).expect("Failed to serialize config"),
        );
    }
}

pub fn reset_clipboard_db_location_disable() {
    let (mut config, data_path) = get_config();
    // copy backup file to default database location
    fs::copy(&config.db, &data_path.db_file_path).expect("Failed to copy database");

    // overwrite config database default location
    config.db = data_path.db_file_path;

    // overwrite config file
    fs::write(
        &data_path.config_file_path,
        serde_json::to_string(&config).expect("Failed to serialize config"),
    )
    .expect("Failed to serialize config");
}
