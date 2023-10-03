use super::hotkey::hotkey_listener::init_hotkey_listener;
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

pub static GLOBAL_EVENTS: [&'static str; 2] = ["window_display_toggle", "recent_clipboards"];

pub static VIEW_MORE_EVENTS: [&'static str; 4] =
    ["sync_clipboard_history", "preferences", "about", "exit"];

pub static SIDEBAR_ICON_EVENTS: [&'static str; 4] = [
    "recent_clipboards",
    "starred_clipboards",
    "history",
    "view_more",
];

pub fn setup(app: &mut tauri::App) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    APP.set(app.handle()).expect("error initializing tauri app");
    let _ = HOTKEY_MANAGER.set(GlobalHotKeyManager::new().unwrap());
    let _ = HOTKEYS.set(Arc::new(Mutex::new(HashMap::new())));
    let _ = CLIPBOARD.set(Arc::new(Mutex::new(Clipboard::new()?)));
    HOTKEY_STOP_TX.set(Mutex::new(None)).unwrap_or_else(|_| {
        panic!("Failed to initialize HOTKEY_STOP_TX");
    });

    create_config();

    let window = app.get_window("main").unwrap();
    let _ = window.set_size(LogicalSize::new(MAIN_WINDOW_X, MAIN_WINDOW_Y));
    #[cfg(any(windows, target_os = "macos"))]
    set_shadow(&window, true).unwrap();
    #[cfg(debug_assertions)]
    {
        window.open_devtools();
    }

    tauri::async_runtime::spawn(async { Master::new(Handler).run() });

    init_hotkey_listener(1);
    init_hotkey_listener(2);

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
