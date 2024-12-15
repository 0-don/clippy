use crate::{
    service::{
        hotkey::with_hotkeys,
        settings::{get_settings_db, sync_clipboard_history_toggle, update_settings_db},
    },
    tauri_config::config::autostart,
};
use common::types::types::CommandError;
use entity::settings::Model;

#[tauri::command]
pub async fn get_settings() -> Result<Model, CommandError> {
    Ok(get_settings_db().await?)
}

#[tauri::command]
pub async fn update_settings(settings: Model) -> Result<(), CommandError> {
    with_hotkeys(false, async move {
        update_settings_db(settings)
            .await
            .expect("Failed to update settings");
    })
    .await;

    Ok(())
}

#[tauri::command]
pub async fn toggle_autostart() -> Result<(), CommandError> {
    Ok(autostart())
}

#[tauri::command]
pub async fn sync_clipboard_history() {
    sync_clipboard_history_toggle().await;
}
