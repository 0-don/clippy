// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod connection;
mod utils;
use commands::clipboard;
use commands::hotkey;
use commands::settings;
use once_cell::sync::OnceCell;
use utils::tray;

pub static APP: OnceCell<tauri::AppHandle> = OnceCell::new();

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard::init())
        .setup(|app| {
            APP.set(app.handle()).expect("error initializing tauri app");
            Ok(())
        })
        .system_tray(tray::system_tray())
        .invoke_handler(tauri::generate_handler![
            clipboard::greet,
            hotkey::get_hotkeys,
            settings::get_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
