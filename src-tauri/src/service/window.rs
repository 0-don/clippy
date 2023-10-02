use std::{fs, path::PathBuf};

use tauri::{Manager, Window};
use tauri_plugin_positioner::{Position, WindowExt};

use crate::{types::types::DataPath, utils::setup::APP};

pub fn init_event() {
    APP.get()
        .unwrap()
        .get_window("main")
        .unwrap()
        .emit("init_listener", Some(()))
        .unwrap();
}

pub fn init_hotkey() {
    APP.get()
        .unwrap()
        .get_window("main")
        .unwrap()
        .emit("init_hotkeys_listener", Some(()))
        .unwrap();
}

pub fn get_main_window() -> Window {
    APP.get().unwrap().get_window("main").unwrap()
}

pub fn toggle_main_window() {
    let window = get_main_window();
    if window.is_visible().unwrap() {
        let _ = window.hide();
    } else {
        let _ = window.move_window(Position::BottomRight);
        let _ = window.show();
        let _ = window.set_focus();
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
