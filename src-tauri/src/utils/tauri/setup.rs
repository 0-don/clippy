use super::config::{create_config, init_config, init_window};
use crate::events::{
    clipboard_events::Handler, hotkey_events::init_hotkey_listener,
    window_events::window_event_listener,
};
use clipboard_master::Master;

// use window_shadows::set_shadow;

pub static GLOBAL_EVENTS: [&'static str; 2] = ["window_display_toggle", "type_clipboard"];

pub fn setup(app: &mut tauri::App) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    create_config();
    init_config(app);
    init_window(app);

    window_event_listener();
    tauri::async_runtime::spawn(async { Master::new(Handler).run() });
    init_hotkey_listener(false);

    Ok(())
}
