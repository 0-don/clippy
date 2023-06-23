use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};

use crate::{
    service::window::{get_data_path, init_hotkey},
    types::types::Config,
    utils::setup::APP,
};

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

// https://docs.rs/dirs_next/
#[tauri::command]
pub fn sync_clipboard_history(dir: Option<String>) {
    let data_path = get_data_path();

    let mut config: Config =
        serde_json::from_str(&read_to_string(&data_path.config_file_path).unwrap()).unwrap();

    if dir.is_some() {
        let dir_file = [dir.as_ref().unwrap(), "clippy.sqlite"]
            .iter()
            .collect::<PathBuf>()
            .to_string_lossy()
            .to_string();

        // copy file from config to dir_file

        fs::copy(&config.db, &dir_file).unwrap();

        config.db = format!("sqlite://{}?mode=rwc", &dir_file);

        let _ = fs::write(
            &data_path.config_file_path,
            serde_json::to_string(&config).unwrap(),
        );
    } else {
    }
}
