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
        .expect("Failed to get MAIN_WINDOW")
        .lock()
        .expect("Failed to lock MAIN_WINDOW")
}

pub fn get_global_hotkey_manager() -> MutexGuard<'static, SafeHotKeyManager> {
    GLOBAL_HOTKEY_MANAGER
        .get()
        .expect("Failed to get GLOBAL_HOTKEY_MANAGER")
        .lock()
        .expect("Failed to lock GLOBAL_HOTKEY_MANAGER")
}

pub fn get_window_hotkey_manager() -> MutexGuard<'static, Option<SafeHotKeyManager>> {
    WINDOW_HOTKEY_MANAGER
        .get()
        .expect("Failed to get WINDOW_HOTKEY_MANAGER")
        .lock()
        .expect("Failed to lock WINDOW_HOTKEY_MANAGER")
}

pub fn get_global_hotkey_store() -> MutexGuard<'static, HashMap<u32, Key>> {
    GLOBAL_HOTKEYS
        .get()
        .expect("Failed to get GLOBAL_HOTKEYS")
        .lock()
        .expect("Failed to lock GLOBAL_HOTKEYS")
}

pub fn get_window_hotkey_store() -> MutexGuard<'static, HashMap<u32, Key>> {
    WINDOW_HOTKEYS
        .get()
        .expect("Failed to get WINDOW_HOTKEYS")
        .lock()
        .expect("Failed to lock WINDOW_HOTKEYS")
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
