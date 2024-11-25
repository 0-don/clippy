use super::global::{get_app, get_main_window};
use crate::{
    commands::settings::get_settings,
    printlog,
    service::{
        global::get_window_stop_tx, hotkey::with_hotkeys, settings::update_settings_synchronize,
    },
    types::types::{Config, DataPath, WindowName},
    utils::hotkey_manager::{register_hotkeys, unregister_hotkeys},
};
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use tauri::{Emitter, Manager, WebviewUrl};
use tauri::{PhysicalPosition, WebviewWindowBuilder};
use tauri_plugin_dialog::DialogExt;

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
        // get_main_window().move_window(Position::Center).unwrap();
        position_window_near_cursor();
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

pub fn position_window_near_cursor() {
    let window = get_main_window();

    if let Ok(cursor_position) = window.cursor_position() {
        let window_size = window.outer_size().unwrap();

        // Get current monitor or fallback to primary
        let current_monitor = window
            .available_monitors()
            .unwrap()
            .into_iter()
            .find(|monitor| {
                let pos = monitor.position();
                let size = monitor.size();
                let bounds = (
                    pos.x as f64,
                    pos.y as f64,
                    pos.x as f64 + size.width as f64,
                    pos.y as f64 + size.height as f64,
                );

                cursor_position.x >= bounds.0
                    && cursor_position.x < bounds.2
                    && cursor_position.y >= bounds.1
                    && cursor_position.y < bounds.3
            })
            .unwrap_or_else(|| window.primary_monitor().unwrap().unwrap());

        let scale_factor = current_monitor.scale_factor();
        let monitor_pos = current_monitor.position();
        let monitor_size = current_monitor.size();

        // Calculate window position with offset
        let pos = PhysicalPosition::new(
            ((cursor_position.x + 10.0) * scale_factor) as i32,
            ((cursor_position.y + 10.0) * scale_factor) as i32,
        );

        // Calculate monitor bounds in physical pixels
        let monitor_bounds = (
            (monitor_pos.x as f64 * scale_factor) as i32,
            (monitor_pos.y as f64 * scale_factor) as i32,
            (monitor_pos.x as f64 * scale_factor + monitor_size.width as f64 * scale_factor) as i32,
            (monitor_pos.y as f64 * scale_factor + monitor_size.height as f64 * scale_factor)
                as i32,
        );

        // Constrain window position within monitor bounds
        let final_pos = PhysicalPosition::new(
            pos.x
                .max(monitor_bounds.0)
                .min(monitor_bounds.2 - window_size.width as i32),
            pos.y
                .max(monitor_bounds.1)
                .min(monitor_bounds.3 - window_size.height as i32),
        );

        window.set_position(final_pos).unwrap();
    }
}

pub fn get_data_path() -> DataPath {
    let config_path = get_app()
        .path()
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

    // Use blocking_pick_folder for synchronous folder selection
    if let Some(dir) = get_app().dialog().file().blocking_pick_folder() {
        // Convert path to string
        let dir = dir.to_string();
        let dir_file = format!("{}/clippy.sqlite", &dir);

        println!("selected dir: {}", dir);

        // check if backup file exists
        if !Path::new(&dir_file).exists() {
            // copy current database to backup location
            let _ = fs::copy(&config.db, &dir_file);
        }

        // overwrite config database location
        config.db = dir_file;

        // overwrite config file
        let _ = fs::write(
            &data_path.config_file_path,
            serde_json::to_string(&config).unwrap(),
        );

        // Now we can await this since we're in an async function
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

pub fn create_about_window() {
    let app = crate::service::global::get_app();

    // Close existing window if it exists
    if let Some(window) = app.get_webview_window("about") {
        let _ = window.close();
    }

    WebviewWindowBuilder::new(app, "about", WebviewUrl::App("pages/about.html".into()))
        .title("About")
        .inner_size(375.0, 600.0)
        .always_on_top(true)
        .build()
        .unwrap();
}

pub fn create_settings_window() {
    let app = crate::service::global::get_app();

    // Close existing window if it exists
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.close();
    }

    WebviewWindowBuilder::new(
        app,
        "settings",
        WebviewUrl::App("pages/settings.html".into()),
    )
    .title("Settings")
    .inner_size(500.0, 450.0)
    .always_on_top(true)
    .build()
    .unwrap();
}

pub fn open_window(window_name: WindowName) {
    match window_name {
        WindowName::About => create_about_window(),
        WindowName::Settings => create_settings_window(),
    }
}
