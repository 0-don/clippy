// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod connection;
mod service;
mod utils;
use commands::{clipboard, hotkey, settings, window};
use tauri_plugin_autostart::MacosLauncher;
use utils::{setup, tray};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(setup::setup)
        .system_tray(tray::system_tray())
        .on_system_tray_event(tray::system_tray_event)
        .invoke_handler(tauri::generate_handler![
            clipboard::get_clipboards,
            clipboard::delete_clipboard,
            clipboard::star_clipboard,
            clipboard::copy_clipboard,
            hotkey::get_hotkeys,
            hotkey::update_hotkey,
            settings::get_settings,
            settings::update_settings,
            window::window_display_toggle,
            window::sync_clipboard_history
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
