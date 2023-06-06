// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod connection;
mod utils;
use commands::clipboard;
use commands::hotkey;
use once_cell::sync::OnceCell;
use utils::system_tray::system_tray;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

pub static APP: OnceCell<tauri::AppHandle> = OnceCell::new();

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            APP.set(app.handle()).expect("error initializing tauri app");
            Ok(())
        })
        .system_tray(system_tray())
        .invoke_handler(tauri::generate_handler![
            clipboard::greet,
            hotkey::get_hotkeys
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
