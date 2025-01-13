use crate::{
    service::{
        hotkey::with_hotkeys,
        settings::{
            change_clipboard_db_location_enable, get_settings_db,
            reset_clipboard_db_location_disable, update_settings_db,
        },
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
pub async fn change_clipboard_db_location() {
    change_clipboard_db_location_enable();
}

#[tauri::command]
pub async fn reset_clipboard_db_location() {
    reset_clipboard_db_location_disable();
}
