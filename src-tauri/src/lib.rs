mod commands;
mod connection;
mod events;
mod service;
mod types;
mod utils;
use commands::{clipboard, hotkey, settings, window};
use tauri_plugin_autostart::MacosLauncher;
use utils::tauri::setup;

#[macro_export]
macro_rules! printlog {
    ($($arg:tt)*) => {
        {
            use chrono::{Local, DateTime};
            let now: DateTime<Local> = Local::now();
            let millis = now.timestamp_subsec_millis();
            println!("{}.{:03}: {}", now.format("%Y-%m-%d %H:%M:%S"), millis, format!($($arg)*));
        }
    };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_dialog::init())
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
            hotkey::start_hotkeys,
            settings::get_settings,
            settings::update_settings,
            settings::toggle_autostart,
            window::window_display_toggle,
            window::get_db_size,
            window::get_db_path,
            window::sync_clipboard_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
