use crate::{
    service::{
        hotkey::with_hotkeys,
        settings::{get_settings_db, update_settings_db},
    },
    types::types::CommandError,
    utils::tauri::config::autostart,
};
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
pub async fn toggle_autostart() -> Result<(), String> {
    Ok(autostart())
}
