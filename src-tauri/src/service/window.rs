use super::global::{get_app, get_main_window};
use crate::{
    commands::settings::get_settings,
    printlog,
    service::{
        global::get_window_stop_tx, hotkey::with_hotkeys, settings::update_settings_synchronize,
    },
    types::types::{Config, DataPath},
    utils::hotkey_manager::{register_hotkeys, unregister_hotkeys},
};
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use tauri::api::dialog::blocking::FileDialogBuilder;
use tauri_plugin_positioner::{Position, WindowExt};

pub fn toggle_main_window() {
    if get_main_window().is_visible().unwrap() {
        printlog!("hiding window");
        if let Some(tx) = get_window_stop_tx().take() {
            let _ = tx.send(());
        }

        get_main_window().hide().unwrap();
        unregister_hotkeys(false);
        get_main_window()
            .emit("set_global_hotkey_event", false)
            .unwrap();
    } else {
        get_main_window()
            .move_window(Position::BottomRight)
            .unwrap();
        get_main_window()
            .emit("change_tab", "recent_clipboards")
            .unwrap();
        get_main_window().show().unwrap();

        register_hotkeys(true);
        get_main_window()
            .emit("set_global_hotkey_event", true)
            .unwrap();

        get_app()
            .run_on_main_thread(|| get_main_window().set_focus().unwrap())
            .unwrap();

        printlog!("displaying window");
    }
}

pub fn get_data_path() -> DataPath {
    let config_path = get_app()
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
