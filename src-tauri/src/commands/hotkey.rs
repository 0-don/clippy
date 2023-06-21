use crate::service::hotkey::get_all_hotkeys_db;
use entity::hotkey::Model;

#[tauri::command]
pub async fn get_hotkeys() -> Result<Vec<Model>, String> {
    let res = get_all_hotkeys_db().await;

    Ok(res.unwrap())
}

#[tauri::command]
pub async fn update_hotkey() -> Result<Vec<Model>, String> {
    let res = get_all_hotkeys_db().await;

    Ok(res.unwrap())
}
