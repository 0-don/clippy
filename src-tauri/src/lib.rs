mod commands;
mod config;
mod events;
mod prelude;
mod service;
mod tao;
mod utils;

use commands::{cipher, clipboard, hotkey, settings, sync, window};
use config::setup;
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Route panics through `log` so tauri-plugin-log writes them to the file too.
    // Without this, panics only hit stderr, which the Windows GUI release discards.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        log::error!("panic: {}", info);
        default_hook(info);
    }));

    // clear_targets() is required: the plugin enables a Stdout target BY DEFAULT, and on
    // the Windows GUI release (windows_subsystem="windows") there is no console, so that
    // default Stdout target aborts logger init and the app exits instantly with no output.
    // We clear it and add only the file target (plus Stdout in debug, where a console exists).
    let mut log_builder = tauri_plugin_log::Builder::new()
        .clear_targets()
        .level(log::LevelFilter::Info)
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::LogDir {
                file_name: Some("clippy".to_string()),
            },
        ));
    #[cfg(debug_assertions)]
    {
        log_builder = log_builder.target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Stdout,
        ));
    }

    let mut builder = tauri::Builder::default()
        .plugin(log_builder.build())
        .plugin(tauri_plugin_clipboard::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ));

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|_app, _argv, _cwd| {}));
    }

    builder
        .setup(setup::setup)
        .invoke_handler(tauri::generate_handler![
            clipboard::get_clipboards,
            clipboard::search_clipboards,
            clipboard::delete_clipboard,
            clipboard::star_clipboard,
            clipboard::rename_clipboard,
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
            settings::change_settings_text_matchers,
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
            cipher::disable_encryption_stream,
            cipher::password_unlock,
            cipher::password_unlock_stream,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
