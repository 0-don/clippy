use crate::{
    service::hotkey::{get_all_hotkeys_db, init_hotkey_window, update_hotkey_db},
    utils::hotkey_manager::{register_hotkeys, unregister_hotkeys, upsert_hotkeys_in_store},
};
use common::types::types::CommandError;
use entity::hotkey::Model;

#[tauri::command]
pub async fn get_hotkeys() -> Result<Vec<Model>, CommandError> {
    Ok(get_all_hotkeys_db().await?)
}

#[tauri::command]
pub async fn update_hotkey(hotkey: Model) {
    unregister_hotkeys(true);

    update_hotkey_db(hotkey)
        .await
        .expect("Failed to update hotkey");

    upsert_hotkeys_in_store()
        .await
        .expect("Failed to upsert hotkeys in store");

    register_hotkeys(true);

    init_hotkey_window();
}

#[tauri::command]
pub async fn stop_hotkeys() {
    unregister_hotkeys(false)
}
