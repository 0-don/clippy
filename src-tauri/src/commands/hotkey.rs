use crate::{
    service::hotkey::{get_all_hotkeys_db, update_hotkey_db},
    utils::{
        hotkey_manager::{register_hotkeys, unregister_hotkeys, upsert_hotkeys_in_store},
        tauri::config::MAIN_WINDOW,
    },
};
use entity::hotkey::Model;

#[tauri::command]
pub async fn get_hotkeys() -> Result<Vec<Model>, String> {
    let res = get_all_hotkeys_db().await;

    Ok(res.unwrap())
}

#[tauri::command]
pub async fn update_hotkey(hotkey: Model) -> Result<Model, String> {
    unregister_hotkeys(true);

    let res = update_hotkey_db(hotkey).await;
    upsert_hotkeys_in_store().await.unwrap();
    MAIN_WINDOW
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .emit("init", ())
        .unwrap();
    register_hotkeys(true);

    Ok(res.unwrap())
}
