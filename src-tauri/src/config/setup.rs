use super::tray::setup_system_tray;
use crate::{
    events::{
        clipboard_events::setup_clipboard_listener, hotkey_events::setup_hotkey_listener,
        window_events::setup_window_event_listener,
    },
    service::{
        encrypt::init_encryption, settings::setup_settings, sync::setup_sync_interval,
        window::setup_window,
    },
    tao::{config::setup_config, tao_constants::setup_globals},
};

pub fn setup(app: &mut tauri::App) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    setup_globals(app);
    setup_config();

    setup_settings();
    setup_window();
    setup_system_tray()?;

    setup_clipboard_listener();
    setup_hotkey_listener();
    setup_window_event_listener();
    setup_sync_interval();

    init_encryption();

    Ok(())
}
