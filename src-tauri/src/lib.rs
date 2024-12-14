mod commands;
mod connection;
mod events;
mod prelude;
mod service;
mod tauri_config;
mod utils;

use commands::{clipboard, hotkey, settings, window};
use tauri_config::setup;
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .setup(setup::setup)
        .invoke_handler(tauri::generate_handler![
            clipboard::get_clipboards,
            clipboard::delete_clipboard,
            clipboard::star_clipboard,
            clipboard::copy_clipboard,
            clipboard::clear_clipboards,
            clipboard::save_clipboard_image,
            hotkey::get_hotkeys,
            hotkey::update_hotkey,
            hotkey::stop_hotkeys,
            settings::get_settings,
            settings::update_settings,
            settings::toggle_autostart,
            window::open_new_window,
            window::open_browser_url,
            window::exit_app,
            window::get_app_version,
            window::get_db_info,
            window::get_db_path,
            window::sync_clipboard_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
