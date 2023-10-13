use crate::define_hotkey_event;
use crate::service::window::get_data_path;
use crate::types::types::{Config, Key};
use arboard::Clipboard;
use global_hotkey::GlobalHotKeyManager;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;
use std::sync::{Arc, Mutex};
use tauri::{LogicalSize, Manager, Window};
use tokio::sync::oneshot;
#[cfg(any(windows, target_os = "macos"))]
use window_shadows::set_shadow;

pub static GLOBAL_EVENTS: [&'static str; 2] = ["window_display_toggle", "type_clipboard"];

pub static MAIN_WINDOW_X: i32 = 375;
pub static MAIN_WINDOW_Y: i32 = 600;

pub static APP: OnceLock<tauri::AppHandle> = OnceLock::new();
pub static MAIN_WINDOW: OnceLock<Arc<Mutex<Window>>> = OnceLock::new();

pub static HOTKEY_MANAGER: OnceLock<GlobalHotKeyManager> = OnceLock::new();
pub static HOTKEY_RUNNING: OnceLock<Arc<Mutex<bool>>> = OnceLock::new();
pub static HOTKEYS: OnceLock<Arc<Mutex<HashMap<u32, Key>>>> = OnceLock::new();
pub static HOTKEY_STOP_TX: OnceLock<Mutex<Option<oneshot::Sender<()>>>> = OnceLock::new();
pub static CLIPBOARD: OnceLock<Arc<Mutex<Clipboard>>> = OnceLock::new();
pub static WINDOW_STOP_TX: OnceLock<Mutex<Option<oneshot::Sender<()>>>> = OnceLock::new();

define_hotkey_event! {
    WindowDisplayToggle => "window_display_toggle",
    TypeClipboard => "type_clipboard",
    ScrollToTop => "scroll_to_top",
    SyncClipboardHistory => "sync_clipboard_history",
    Preferences => "preferences",
    About => "about",
    Exit => "exit",
    RecentClipboard => "recent_clipboards",
    StarredClipboard => "starred_clipboards",
    History => "history",
    ViewMore => "view_more",
    // Digit1 => "digit_1",
    // Digit2 => "digit_2",
    // Digit3 => "digit_3",
    // Digit4 => "digit_4",
    // Digit5 => "digit_5",
    // Digit6 => "digit_6",
    // Digit7 => "digit_7",
    // Digit8 => "digit_8",
    // Digit9 => "digit_9",
    // Num1 => "num_1",
    // Num2 => "num_2",
    // Num3 => "num_3",
    // Num4 => "num_4",
    // Num5 => "num_5",
    // Num6 => "num_6",
    // Num7 => "num_7",
    // Num8 => "num_8",
    // Num9 => "num_9",
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

pub fn init_globals(app: &mut tauri::App) {
    APP.set(app.handle())
        .unwrap_or_else(|_| panic!("Failed to initialize APP"));
    HOTKEY_MANAGER
        .set(GlobalHotKeyManager::new().unwrap())
        .unwrap_or_else(|_| panic!("Failed to initialize HOTKEY_MANAGER"));
    HOTKEY_RUNNING
        .set(Arc::new(Mutex::new(false)))
        .unwrap_or_else(|_| panic!("Failed to initialize HOTKEY_RUNNING"));
    HOTKEYS
        .set(Arc::new(Mutex::new(HashMap::new())))
        .unwrap_or_else(|_| panic!("Failed to initialize HOTKEYS"));
    CLIPBOARD
        .set(Arc::new(Mutex::new(Clipboard::new().unwrap())))
        .unwrap_or_else(|_| panic!("Failed to initialize CLIPBOARD"));
    HOTKEY_STOP_TX
        .set(Mutex::new(None))
        .unwrap_or_else(|_| panic!("Failed to initialize HOTKEY_STOP_TX"));
    WINDOW_STOP_TX
        .set(Mutex::new(None))
        .unwrap_or_else(|_| panic!("Failed to initialize WINDOW_STOP_TX"));
}

pub fn init_window(app: &mut tauri::App) {
    let window: tauri::Window = app.get_window("main").unwrap();
    let _ = window.set_size(LogicalSize::new(MAIN_WINDOW_X, MAIN_WINDOW_Y));
    #[cfg(any(windows, target_os = "macos"))]
    set_shadow(&window, true).unwrap();
    #[cfg(debug_assertions)]
    {
        window.open_devtools();
    }
    MAIN_WINDOW
        .set(Arc::new(Mutex::new(window)))
        .unwrap_or_else(|_| panic!("Failed to initialize MAIN_WINDOW"));
}
