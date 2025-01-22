mod commands;
mod config;
mod events;
mod prelude;
mod service;
mod utils;

use commands::{cipher, clipboard, hotkey, settings, sync, window};
use config::setup;
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
            //
            hotkey::get_hotkeys,
            hotkey::update_hotkey,
            hotkey::stop_hotkeys,
            //
            settings::get_settings,
            settings::update_settings,
            settings::toggle_autostart,
            settings::change_clipboard_db_location,
            settings::reset_clipboard_db_location,
            //
            window::open_new_window,
            window::open_browser_url,
            window::exit_app,
            window::get_app_version,
            window::get_db_info,
            window::get_db_path,
            window::get_config_path,
            window::open_folder,
            //
            sync::sync_authenticate_toggle,
            sync::sync_limit_change,
            //
            //
            cipher::enable_encryption,
            cipher::disable_encryption,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
