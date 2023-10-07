use crate::{
    service::hotkey::{get_all_hotkeys_db, update_hotkey_db},
    utils::hotkey_manager::{register_hotkeys, unregister_hotkeys},
};
use entity::hotkey::Model;

#[tauri::command]
pub async fn get_hotkeys() -> Result<Vec<Model>, String> {
    let res = get_all_hotkeys_db().await;

    Ok(res.unwrap())
}

#[tauri::command]
pub async fn update_hotkey(hotkey: Model) -> Result<Model, String> {
    unregister_hotkeys(true);
    let res = update_hotkey_db(hotkey).await;
    register_hotkeys(true);

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
