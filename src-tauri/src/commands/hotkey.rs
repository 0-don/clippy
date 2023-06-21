use crate::service::{
    hotkey::{get_all_hotkeys_db, update_hotkey_db},
    window::init_event,
};
use entity::hotkey::Model;

#[tauri::command]
pub async fn get_hotkeys() -> Result<Vec<Model>, String> {
    let res = get_all_hotkeys_db().await;

    Ok(res.unwrap())
}

#[tauri::command]
pub async fn update_hotkey(hotkey: Model) -> Result<Model, String> {
    let res = update_hotkey_db(hotkey).await;

    init_event();

    Ok(res.unwrap())
}
