use crate::{service::hotkey::{get_all_hotkeys_db, update_hotkey_db}, events::hotkey_events::init_hotkey_listener, utils::hotkey_manager::{unregister_hotkeys, register_hotkeys}};
use entity::hotkey::Model;

#[tauri::command]
pub async fn get_hotkeys() -> Result<Vec<Model>, String> {
    let res = get_all_hotkeys_db().await;

    Ok(res.unwrap())
}

#[tauri::command]
pub async fn update_hotkey(hotkey: Model) -> Result<Model, String> {
    let res = update_hotkey_db(hotkey).await;

    init_hotkey_listener(false);

    Ok(res.unwrap())
}

#[tauri::command]
pub async fn stop_hotkeys() -> Result<(), String> {
    Ok(unregister_hotkeys(false))
}

#[tauri::command]
pub async fn start_hotkeys() -> Result<(), String> {
    Ok(register_hotkeys(true))
}
