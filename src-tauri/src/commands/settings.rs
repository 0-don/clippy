use entity::settings::Model;

use crate::{service::settings::{update_settings_db, get_settings_db}, events::hotkey_events::init_hotkey_listener};

#[tauri::command]
pub async fn get_settings() -> Result<Model, String> {
    let res = get_settings_db().await;

    Ok(res.unwrap())
}

#[tauri::command]
pub async fn update_settings(settings: Model) -> Result<Model, String> {
    let res = update_settings_db(settings).await;

    init_hotkey_listener(false);

    Ok(res.unwrap())
}
