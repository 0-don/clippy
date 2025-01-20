use common::types::{enums::WebWindow, hotkey::SafeHotKeyManager, types::Key};
use global_hotkey::GlobalHotKeyManager;
use sea_orm::Iden;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};
use tauri::{Manager, WebviewWindow};
use tokio::sync::oneshot;

pub static APP: OnceLock<tauri::AppHandle> = OnceLock::new();
pub static MAIN_WINDOW: OnceLock<Arc<Mutex<WebviewWindow>>> = OnceLock::new();

pub static HOTKEY_MANAGER: OnceLock<Arc<Mutex<SafeHotKeyManager>>> = OnceLock::new();
pub static HOTKEY_RUNNING: OnceLock<Arc<Mutex<bool>>> = OnceLock::new();
pub static HOTKEYS: OnceLock<Arc<Mutex<HashMap<u32, Key>>>> = OnceLock::new();
pub static HOTKEY_STOP_TX: OnceLock<Mutex<Option<oneshot::Sender<()>>>> = OnceLock::new();
pub static WINDOW_STOP_TX: OnceLock<Mutex<Option<oneshot::Sender<()>>>> = OnceLock::new();

pub fn init_globals(app: &mut tauri::App) {
    APP.set(app.handle().clone())
        .unwrap_or_else(|_| panic!("Failed to initialize APP"));
    HOTKEY_MANAGER
        .set(Arc::new(Mutex::new(SafeHotKeyManager::new(
            GlobalHotKeyManager::new().expect("Failed to initialize GlobalHotKeyManager"),
        ))))
        .unwrap_or_else(|_| panic!("Failed to initialize HOTKEY_MANAGER"));
    HOTKEY_RUNNING
        .set(Arc::new(Mutex::new(false)))
        .unwrap_or_else(|_| panic!("Failed to initialize HOTKEY_RUNNING"));
    HOTKEYS
        .set(Arc::new(Mutex::new(HashMap::new())))
        .unwrap_or_else(|_| panic!("Failed to initialize HOTKEYS"));
    HOTKEY_STOP_TX
        .set(Mutex::new(None))
        .unwrap_or_else(|_| panic!("Failed to initialize HOTKEY_STOP_TX"));
    WINDOW_STOP_TX
        .set(Mutex::new(None))
        .unwrap_or_else(|_| panic!("Failed to initialize WINDOW_STOP_TX"));
    MAIN_WINDOW
        .set(Arc::new(Mutex::new(
            app.get_webview_window(WebWindow::Main.to_string().as_str())
                .expect("Failed to get window"),
        )))
        .expect("Failed to set main window");
}
