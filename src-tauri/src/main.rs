// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod connection;
mod utils;

use core::time::Duration;
use std::thread;

use commands::clipboard;
use commands::hotkey;
use utils::system_tray::system_tray;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .on_page_load(|window, _| {
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(1));
                window
                    .emit(
                        "click",
                        Payload {
                            message: "Tauri is awesome!w".into(),
                        },
                    )
                    .unwrap();
            });
        })
        .system_tray(system_tray())
        .invoke_handler(tauri::generate_handler![clipboard::greet])
        .invoke_handler(tauri::generate_handler![hotkey::get_hotkeys])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
