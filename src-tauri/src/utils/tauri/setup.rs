use super::{
    config::{create_config, init_globals},
    tray::create_system_tray,
};
use crate::{
    events::{
        clipboard_events::init_clipboard_listener, hotkey_events::init_hotkey_listener,
        window_events::window_event_listener,
    },
    service::window::init_window,
};

pub fn setup(app: &mut tauri::App) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    init_globals(app);
    init_window(app);
    init_clipboard_listener(app);
    create_config();

    window_event_listener();

    init_hotkey_listener();

    let _tray = create_system_tray(app)?;

    Ok(())
}
