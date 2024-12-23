use super::{
    config::{create_config, init_globals},
    tray::init_system_tray,
};
use crate::{
    events::{
        clipboard_events::init_clipboard_listener, hotkey_events::init_hotkey_listener,
        window_events::init_window_event_listener,
    },
    service::{settings::init_settings, window::init_window},
};

pub fn setup(app: &mut tauri::App) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    init_globals(app);
    create_config();

    init_settings();
    init_window();
    init_system_tray()?;

    init_clipboard_listener();
    init_hotkey_listener();
    init_window_event_listener();

    Ok(())
}
