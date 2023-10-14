use crate::{
    commands::settings::get_settings,
    service::{hotkey::with_hotkeys, settings::update_settings_synchronize},
    types::types::{Config, DataPath},
    utils::{
        hotkey_manager::{register_hotkeys, unregister_hotkeys},
        tauri::config::{APP, HOTKEY_RUNNING, MAIN_WINDOW, WINDOW_STOP_TX},
    },
};
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri_plugin_positioner::{Position, WindowExt};

pub fn toggle_main_window() {
    let window = MAIN_WINDOW.get().unwrap().lock().unwrap();
    if window.is_visible().unwrap() {
        // if let Some(tx) = WINDOW_STOP_TX.get().unwrap().lock().unwrap().take() {
        //     let _ = tx.send(());
        // }

        window.hide().unwrap();
        // unregister_hotkeys(false);
        // window.emit("set_global_hotkey_event", false).unwrap();
        // *HOTKEY_RUNNING.get().unwrap().lock().unwrap() = false;
    } else {
        window.move_window(Position::BottomRight).unwrap();
        // window.emit("change_tab", "recent_clipboards").unwrap();
        window.show().unwrap();
        if !cfg!(target_os = "linux") {
            window.set_focus().unwrap();
        }
        // register_hotkeys(true);
        // window.emit("set_global_hotkey_event", true).unwrap();
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

pub fn get_config() -> (Config, DataPath) {
    let data_path = get_data_path();

    let json = std::fs::read_to_string(&data_path.config_file_path).unwrap();

    let config: Config = serde_json::from_str(&json).unwrap();

    (config, data_path)
}

pub async fn sync_clipboard_history_enable() {
    // get local config from app data
    let (mut config, data_path) = get_config();
    let dir = FileDialogBuilder::new().pick_folder();

    println!("config: {:?}", dir);
    // check if user disabled backup or not
    if dir.is_some() {
        // path to backup file
        let dir: String = dir.unwrap().to_string_lossy().to_string();
        let dir_file = format!("{}/clippy.sqlite", &dir);

        println!("selected dir: {}", dir);

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

        update_settings_synchronize(true).await.unwrap();
    }
}

pub async fn sync_clipboard_history_disable() {
    let (mut config, data_path) = get_config();
    // copy backup file to default database location
    let _ = fs::copy(&config.db, &data_path.db_file_path);

    // overwrite config database default location
    config.db = data_path.db_file_path;

    // overwrite config file
    let _ = fs::write(
        &data_path.config_file_path,
        serde_json::to_string(&config).unwrap(),
    );

    update_settings_synchronize(false).await.unwrap();
}

pub async fn sync_clipboard_history_toggle() {
    let settings = get_settings().await.unwrap();

    with_hotkeys(false, async move {
        if settings.synchronize {
            sync_clipboard_history_disable().await;
        } else {
            sync_clipboard_history_enable().await;
        }
    })
    .await;
}
