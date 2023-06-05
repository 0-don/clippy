// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod connection;
mod utils;

use commands::clipboard;
use commands::hotkey;
use utils::system_tray::system_tray;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .system_tray(system_tray())
        .invoke_handler(tauri::generate_handler![clipboard::greet])
        .invoke_handler(tauri::generate_handler![hotkey::get_hotkeys])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
