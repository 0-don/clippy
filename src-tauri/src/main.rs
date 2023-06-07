// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod connection;
mod utils;

use commands::{clipboard, hotkey, settings, window};
use utils::{setup, tray};

use tauri_plugin_autostart::MacosLauncher;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(setup::setup)
        .system_tray(tray::system_tray())
        .on_system_tray_event(tray::system_tray_event)
        .invoke_handler(tauri::generate_handler![
            clipboard::insert_clipboard,
            hotkey::get_hotkeys,
            settings::get_settings,
            window::window_on_mouse,
            window::is_production,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
