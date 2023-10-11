use entity::settings::Model;

use crate::{
    service::settings::{get_settings_db, update_settings_db},
    utils::hotkey_manager::{register_hotkeys, unregister_hotkeys, upsert_hotkeys_in_store},
};

#[tauri::command]
pub async fn get_settings() -> Result<Model, String> {
    let res = get_settings_db().await;

    Ok(res.unwrap())
}

#[tauri::command]
pub async fn update_settings(settings: Model) -> Result<Model, String> {
    unregister_hotkeys(true);

    let res = update_settings_db(settings).await;

    upsert_hotkeys_in_store().await.unwrap();

    register_hotkeys(false);

    Ok(res.unwrap())
}
