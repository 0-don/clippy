use super::global::{get_app_window, get_main_window};
use crate::{
    connection,
    utils::hotkey_manager::{register_hotkeys, unregister_hotkeys, upsert_hotkeys_in_store},
};
use core::future::Future;
use entity::hotkey::{self, ActiveModel, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
use tauri::Emitter;

pub async fn get_all_hotkeys_db() -> Result<Vec<Model>, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let hotkeys = hotkey::Entity::find().all(&db).await?;

    Ok(hotkeys)
}

pub async fn update_hotkey_db(hotkey: Model) -> Result<Model, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let active_model: ActiveModel = hotkey.into();

    let updated_hotkey = hotkey::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    Ok(updated_hotkey)
}

pub async fn with_hotkeys<T, F>(register_all: bool, action: F) -> T
where
    F: Future<Output = T>,
{
    unregister_hotkeys(true);

    let result = action.await;
    upsert_hotkeys_in_store().await.unwrap();

    register_hotkeys(register_all);
    get_main_window().emit("init", ()).unwrap();
    get_app_window("settings").emit("init", ()).unwrap();

    result
}
