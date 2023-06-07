// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod connection;
mod utils;
use commands::{clipboard, hotkey, settings, window};

use tauri::Manager;
use tauri::SystemTrayEvent;
use tauri_plugin_positioner::{on_tray_event, Position, WindowExt};

use utils::{setup, tray};

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_positioner::init())
        .setup(setup::setup)
        .system_tray(tray::system_tray())
        .on_system_tray_event(|app, event| {
            on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let win = app.get_window("main").unwrap();
                    let _ = win.move_window(Position::TrayCenter);
                    let _ = win.show();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            clipboard::greet,
            hotkey::get_hotkeys,
            settings::get_settings,
            window::window_on_mouse,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
