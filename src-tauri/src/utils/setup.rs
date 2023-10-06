use super::hotkey::hotkey_listener::init_hotkey_listener;
use crate::define_hotkey_event;
use crate::types::types::Key;
use crate::{
    service::window::get_data_path, types::types::Config,
    utils::clipboard::clipboard_handler::Handler,
};
use arboard::Clipboard;
use clipboard_master::Master;
use global_hotkey::GlobalHotKeyManager;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{fs, path::Path, sync::OnceLock};
use tauri::{LogicalSize, Manager};
use tokio::sync::oneshot;
// use window_shadows::set_shadow;

pub static MAIN_WINDOW_X: i32 = 375;
pub static MAIN_WINDOW_Y: i32 = 600;

pub static APP: OnceLock<tauri::AppHandle> = OnceLock::new();

pub static HOTKEY_MANAGER: OnceLock<GlobalHotKeyManager> = OnceLock::new();
pub static HOTKEYS: OnceLock<Arc<Mutex<HashMap<u32, Key>>>> = OnceLock::new();
pub static HOTKEY_STOP_TX: OnceLock<Mutex<Option<oneshot::Sender<()>>>> = OnceLock::new();
pub static CLIPBOARD: OnceLock<Arc<Mutex<Clipboard>>> = OnceLock::new();

define_hotkey_event! {
    WindowDisplayToggle => "window_display_toggle",
    TypeClipboard => "type_clipboard",
    SyncClipboardHistory => "sync_clipboard_history",
    Preferences => "preferences",
    About => "about",
    Exit => "exit",
    RecentClipboard => "recent_clipboards",
    StarredClipboard => "starred_clipboards",
    History => "history",
    ViewMore => "view_more",
    Key1 => "key_1",
    Key2 => "key_2",
    Key3 => "key_3",
    Key4 => "key_4",
    Key5 => "key_5",
    Key6 => "key_6",
    Key7 => "key_7",
    Key8 => "key_8",
    Key9 => "key_9",
}

pub static GLOBAL_EVENTS: [&'static str; 2] = ["window_display_toggle", "type_clipboard"];

// pub static VIEW_MORE_EVENTS: [&'static str; 4] =
//     ["sync_clipboard_history", "preferences", "about", "exit"];

// pub static SIDEBAR_ICON_EVENTS: [&'static str; 4] = [
//     "recent_clipboards",
//     "starred_clipboards",
//     "history",
//     "view_more",
// ];

pub fn setup(app: &mut tauri::App) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    APP.set(app.handle()).expect("error initializing tauri app");
    let _ = HOTKEY_MANAGER.set(GlobalHotKeyManager::new().unwrap());
    let _ = HOTKEYS.set(Arc::new(Mutex::new(HashMap::new())));
    let _ = CLIPBOARD.set(Arc::new(Mutex::new(Clipboard::new()?)));
    HOTKEY_STOP_TX.set(Mutex::new(None)).unwrap_or_else(|_| {
        panic!("Failed to initialize HOTKEY_STOP_TX");
    });

    create_config();

    let window: tauri::Window = app.get_window("main").unwrap();
    let _ = window.set_size(LogicalSize::new(MAIN_WINDOW_X, MAIN_WINDOW_Y));
    #[cfg(any(windows, target_os = "macos"))]
    set_shadow(&window, true).unwrap();
    #[cfg(debug_assertions)]
    {
        window.open_devtools();
    }

    tauri::async_runtime::spawn(async { Master::new(Handler).run() });

    init_hotkey_listener(false);

    Ok(())
}

pub fn create_config() {
    let data_path = get_data_path();

    if Path::new(&data_path.config_file_path).exists() {
        return;
    }

    let config = Config {
        db: format!("{}", &data_path.db_file_path),
    };

    let _ = fs::write(
        &data_path.config_file_path,
        serde_json::to_string(&config).unwrap(),
    );
}
