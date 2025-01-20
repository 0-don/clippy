use crate::{
    service::hotkey::{get_all_hotkeys_db, init_hotkey_window, update_hotkey_db},
    utils::hotkey_manager::unregister_hotkeys,
};
use common::types::types::CommandError;
use entity::hotkey::Model;

#[tauri::command]
pub async fn get_hotkeys() -> Result<Vec<Model>, CommandError> {
    Ok(get_all_hotkeys_db().await?)
}

#[tauri::command]
pub async fn update_hotkey(hotkey: Model) {
    update_hotkey_db(hotkey)
        .await
        .expect("Failed to update hotkey");

    init_hotkey_window();
}

#[tauri::command]
pub async fn stop_hotkeys() {
    unregister_hotkeys(false)
}
