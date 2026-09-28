use crate::{
    service::hotkey::{get_all_hotkeys_db, init_hotkey_window, update_hotkey_db},
    service::keyboard_layout::get_keyboard_layout_map,
    utils::hotkey_manager::{reload_global_hotkeys, unregister_hotkeys},
};
use common::types::types::CommandError;
use entity::hotkey::Model;
use std::collections::HashMap;

#[tauri::command]
pub async fn get_hotkeys() -> Result<Vec<Model>, CommandError> {
    Ok(get_all_hotkeys_db().await?)
}

#[tauri::command]
pub async fn update_hotkey(hotkey: Model) {
    update_hotkey_db(hotkey)
        .await
        .expect("Failed to update hotkey");

    reload_global_hotkeys().await;

    init_hotkey_window();
}

#[tauri::command]
pub fn get_keyboard_layout() -> HashMap<String, String> {
    get_keyboard_layout_map()
}

#[tauri::command]
pub async fn stop_hotkeys() {
    unregister_hotkeys(false)
}
