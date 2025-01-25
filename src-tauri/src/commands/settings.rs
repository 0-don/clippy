use crate::{service::settings::{autostart, get_settings_db, update_settings_db}, tao::config::{change_clipboard_db_location_enable, reset_clipboard_db_location_disable}};
use common::types::types::CommandError;
use entity::settings::Model;

#[tauri::command]
pub async fn get_settings() -> Result<Model, CommandError> {
    Ok(get_settings_db().await?)
}

#[tauri::command]
pub async fn update_settings(settings: Model) {
    update_settings_db(settings)
        .await
        .expect("Failed to update settings");
}

#[tauri::command]
pub async fn toggle_autostart() {
    autostart()
}

#[tauri::command]
pub async fn change_clipboard_db_location() {
    change_clipboard_db_location_enable();
}

#[tauri::command]
pub async fn reset_clipboard_db_location() {
    reset_clipboard_db_location_disable();
}
