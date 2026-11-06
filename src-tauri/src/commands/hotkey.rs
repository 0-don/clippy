use crate::{
    service::hotkey::{get_all_hotkeys_db, init_hotkey_window, update_hotkey_db},
    service::keyboard_layout::get_keyboard_layout_map,
    utils::hotkey_manager::{reload_global_hotkeys, unregister_hotkeys},
};
use common::{
    io::keyboard::default_hotkey,
    types::{enums::HotkeyEvent, types::CommandError},
};
use entity::hotkey::Model;
use crate::prelude::*;
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
pub async fn reset_hotkey(hotkey: Model) -> Result<Model, CommandError> {
    let (ctrl, key) = HotkeyEvent::iter()
        .find(|event| event.to_string() == hotkey.event)
        .and_then(|event| default_hotkey(&event))
        .ok_or_else(|| CommandError::new("Hotkey has no default"))?;

    let hotkey = Model {
        ctrl,
        alt: false,
        shift: false,
        super_key: false,
        key: key.to_string(),
        status: true,
        ..hotkey
    };
    update_hotkey(hotkey.clone()).await;
    Ok(hotkey)
}

#[tauri::command]
pub fn get_keyboard_layout() -> HashMap<String, String> {
    get_keyboard_layout_map()
}

#[tauri::command]
pub async fn stop_hotkeys() {
    unregister_hotkeys(false)
}
