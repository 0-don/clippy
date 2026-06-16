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

/// On the Windows GUI release (`windows_subsystem="windows"`) the process starts
/// with no console, so stdout/stderr go nowhere. If the app was launched FROM a
/// terminal, attach to that parent console so our logs become visible there.
/// When double-clicked there is no parent console and this is a harmless no-op.
/// Returns true if a console is attached (i.e. it's safe to add a Stdout target).
#[cfg(windows)]
fn attach_parent_console() -> bool {
    use windows_sys::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
    // AttachConsole returns 0 (FALSE) when there is no parent console to attach to.
    unsafe { AttachConsole(ATTACH_PARENT_PROCESS) != 0 }
}

#[cfg(not(windows))]
fn attach_parent_console() -> bool {
    // Other platforms keep their stdout when launched from a terminal.
    true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Route panics through `log` so tauri-plugin-log writes them to the file too.
    // Without this, panics only hit stderr, which the Windows GUI release discards.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        log::error!("panic: {}", info);
        default_hook(info);
    }));

    // Decide whether stdout logging is safe. A console exists when running a debug
    // build, on non-Windows, or when a Windows GUI release was launched from a
    // terminal (attach_parent_console succeeds). Without a console, adding the
    // plugin's Stdout target aborts logger init and the app exits instantly.
    let has_console = cfg!(debug_assertions) || attach_parent_console();

    // clear_targets() is required: the plugin enables a Stdout target BY DEFAULT,
    // which is unsafe on a console-less Windows GUI release. We start from a clean
    // slate with the file target, then add Stdout only when a console is present.
    let mut log_builder = tauri_plugin_log::Builder::new()
        .clear_targets()
        .level(log::LevelFilter::Info)
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::LogDir {
                file_name: Some("clippy".to_string()),
            },
        ));
    if has_console {
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
            settings::get_os,
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
