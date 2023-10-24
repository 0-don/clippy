use crate::{
    service::{
        hotkey::with_hotkeys,
        settings::{get_settings_db, update_settings_db},
    },
    utils::tauri::config::autostart,
};
use entity::settings::Model;

#[tauri::command]
pub async fn get_settings() -> Result<Model, String> {
    let res = get_settings_db().await;

    Ok(res.unwrap())
}

#[tauri::command]
pub async fn update_settings(settings: Model) -> Result<(), String> {
    with_hotkeys(false, async move {
        update_settings_db(settings).await.unwrap();
    })
    .await;

    Ok(())
}

#[tauri::command]
pub async fn toggle_autostart() -> Result<(), String> {
    autostart();

    Ok(())
}
