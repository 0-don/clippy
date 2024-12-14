use crate::commands::settings::get_settings;
use crate::service::global::get_app;
use crate::service::settings::get_data_path;
use common::types::hotkey::SafeHotKeyManager;
use common::types::types::{Config, Key};
use global_hotkey::GlobalHotKeyManager;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;
use std::sync::{Arc, Mutex};
use tauri::{Manager, WebviewWindow};
use tauri_plugin_autostart::AutoLaunchManager;
use tokio::sync::oneshot;

pub static GLOBAL_EVENTS: [&'static str; 2] = ["window_display_toggle", "type_clipboard"];

pub static MAIN_WINDOW_X: i32 = 375;
pub static MAIN_WINDOW_Y: i32 = 600;
pub static MAX_IMAGE_SIZE: u32 = 1280;

pub static APP: OnceLock<tauri::AppHandle> = OnceLock::new();
pub static MAIN_WINDOW: OnceLock<Arc<Mutex<WebviewWindow>>> = OnceLock::new();

pub static HOTKEY_MANAGER: OnceLock<Arc<Mutex<SafeHotKeyManager>>> = OnceLock::new();
pub static HOTKEY_RUNNING: OnceLock<Arc<Mutex<bool>>> = OnceLock::new();
pub static HOTKEYS: OnceLock<Arc<Mutex<HashMap<u32, Key>>>> = OnceLock::new();
pub static HOTKEY_STOP_TX: OnceLock<Mutex<Option<oneshot::Sender<()>>>> = OnceLock::new();
pub static WINDOW_STOP_TX: OnceLock<Mutex<Option<oneshot::Sender<()>>>> = OnceLock::new();

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
        serde_json::to_string(&config).expect("Failed to serialize config"),
    );
}

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
}

pub fn autostart() {
    tauri::async_runtime::spawn(async {
        let app: &tauri::AppHandle = get_app();
        let settings = get_settings().await.expect("Failed to get settings");
        let manager: tauri::State<'_, AutoLaunchManager> = app.state::<AutoLaunchManager>();

        // Use the manager as needed
        if settings.startup && !manager.is_enabled().expect("Failed to check auto-launch") {
            manager.enable().expect("Failed to enable auto-launch");
        } else {
            manager.disable().expect("Failed to disable auto-launch");
        }
    });
}