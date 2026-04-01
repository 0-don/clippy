use super::tao_constants::{
    APP, CLIPBOARD_CACHE, GLOBAL_HOTKEYS, GLOBAL_HOTKEY_MANAGER, HOTKEY_RUNNING, HOTKEY_STOP_TX,
    MAIN_WINDOW, WINDOW_STOP_TX,
};
use crate::tao::tao_constants::{WINDOW_HOTKEYS, WINDOW_HOTKEY_MANAGER};
use common::types::{hotkey::SafeHotKeyManager, orm_query::FullClipboardDto, types::Key};
use moka::sync::Cache;
use std::{collections::HashMap, sync::MutexGuard, time::Duration};
use tauri::{AppHandle, WebviewWindow};
use tokio::sync::oneshot;

pub fn get_main_window() -> MutexGuard<'static, WebviewWindow> {
    MAIN_WINDOW
        .get()
        .expect("MAIN_WINDOW not initialized")
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

pub fn get_global_hotkey_manager() -> MutexGuard<'static, SafeHotKeyManager> {
    GLOBAL_HOTKEY_MANAGER
        .get()
        .expect("GLOBAL_HOTKEY_MANAGER not initialized")
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

pub fn get_window_hotkey_manager() -> MutexGuard<'static, Option<SafeHotKeyManager>> {
    WINDOW_HOTKEY_MANAGER
        .get()
        .expect("WINDOW_HOTKEY_MANAGER not initialized")
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

pub fn get_global_hotkey_store() -> MutexGuard<'static, HashMap<u32, Key>> {
    GLOBAL_HOTKEYS
        .get()
        .expect("GLOBAL_HOTKEYS not initialized")
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

pub fn get_window_hotkey_store() -> MutexGuard<'static, HashMap<u32, Key>> {
    WINDOW_HOTKEYS
        .get()
        .expect("WINDOW_HOTKEYS not initialized")
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

pub fn get_window_stop_tx() -> MutexGuard<'static, Option<oneshot::Sender<()>>> {
    WINDOW_STOP_TX
        .get()
        .expect("WINDOW_STOP_TX not initialized")
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

pub fn get_hotkey_stop_tx() -> MutexGuard<'static, Option<oneshot::Sender<()>>> {
    HOTKEY_STOP_TX
        .get()
        .expect("HOTKEY_STOP_TX not initialized")
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

pub fn get_hotkey_running() -> MutexGuard<'static, bool> {
    HOTKEY_RUNNING
        .get()
        .expect("HOTKEY_RUNNING not initialized")
        .lock()
        .unwrap_or_else(|e| e.into_inner())
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
