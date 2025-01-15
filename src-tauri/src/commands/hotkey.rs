use crate::{
    service::{
        hotkey::{get_all_hotkeys_db, update_hotkey_db},
        settings::init_window_settings,
    },
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
    init_window_settings(async move {
        update_hotkey_db(hotkey)
            .await
            .expect("Failed to update hotkey");
    })
    .await;
}

#[tauri::command]
pub async fn stop_hotkeys() {
    unregister_hotkeys(false)
}
