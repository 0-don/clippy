use super::config::{create_config, init_globals, init_window};
use crate::events::{
    clipboard_events::Handler, hotkey_events::init_hotkey_listener,
    window_events::window_event_listener,
};
use clipboard_master::Master;

pub fn setup(app: &mut tauri::App) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    init_globals(app);
    init_window(app);

    create_config();

    window_event_listener();
    tauri::async_runtime::spawn(async { Master::new(Handler).run() });
    init_hotkey_listener(false);

    Ok(())
}
