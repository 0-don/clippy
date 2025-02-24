use super::tao_constants::{
    APP, CLIPBOARD_CACHE, HOTKEYS, HOTKEY_MANAGER, HOTKEY_RUNNING, HOTKEY_STOP_TX, MAIN_WINDOW,
    WINDOW_STOP_TX,
};
use common::types::{hotkey::SafeHotKeyManager, orm_query::FullClipboardDto, types::Key};
use moka::sync::Cache;
use std::{collections::HashMap, sync::MutexGuard, time::Duration};
use tauri::{AppHandle, WebviewWindow};
use tokio::sync::oneshot;

pub fn get_main_window() -> MutexGuard<'static, WebviewWindow> {
    MAIN_WINDOW
        .get()
        .expect("Failed to get MAIN_WINDOW")
        .lock()
        .expect("Failed to lock MAIN_WINDOW")
}

pub fn get_hotkey_manager() -> MutexGuard<'static, SafeHotKeyManager> {
    HOTKEY_MANAGER
        .get()
        .expect("Failed to get HOTKEY_MANAGER")
        .lock()
        .expect("Failed to lock HOTKEY_MANAGER")
}

pub fn get_hotkey_store() -> MutexGuard<'static, HashMap<u32, Key>> {
    HOTKEYS
        .get()
        .expect("Failed to get HOTKEYS")
        .lock()
        .expect("Failed to lock HOTKEYS")
}

pub fn get_window_stop_tx() -> MutexGuard<'static, Option<oneshot::Sender<()>>> {
    WINDOW_STOP_TX
        .get()
        .expect("Failed to get WINDOW_STOP_TX")
        .lock()
        .expect("Failed to lock WINDOW_STOP_TX")
}

pub fn get_hotkey_stop_tx() -> MutexGuard<'static, Option<oneshot::Sender<()>>> {
    HOTKEY_STOP_TX
        .get()
        .expect("Failed to get HOTKEY_STOP_TX")
        .lock()
        .expect("Failed to lock HOTKEY_STOP_TX")
}

pub fn get_hotkey_running() -> MutexGuard<'static, bool> {
    HOTKEY_RUNNING
        .get()
        .expect("Failed to get HOTKEY_RUNNING")
        .lock()
        .expect("Failed to lock HOTKEY_RUNNING")
}

pub fn get_app() -> &'static AppHandle {
    APP.get().expect("Failed to get APP")
}

pub fn get_cache() -> &'static Cache<String, Vec<FullClipboardDto>> {
    CLIPBOARD_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(Duration::from_secs(300)) // 5 minutes TTL
            .build()
    })
}
