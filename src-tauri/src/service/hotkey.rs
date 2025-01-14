use crate::prelude::*;
use crate::utils::hotkey_manager::{register_hotkeys, unregister_hotkeys, upsert_hotkeys_in_store};
use common::types::enums::{ListenEvent, WebWindow};
use core::future::Future;
use entity::hotkey::{self, ActiveModel, Model};
use sea_orm::{ActiveModelTrait, EntityTrait};
use tao::connection::db;
use tao::global::{get_app, get_main_window};
use tauri::{Emitter, Manager};

pub async fn get_all_hotkeys_db() -> Result<Vec<Model>, DbErr> {
    let db: DatabaseConnection = db().await?;

    let hotkeys = hotkey::Entity::find().all(&db).await?;

    Ok(hotkeys)
}

pub async fn update_hotkey_db(hotkey: Model) -> Result<Model, DbErr> {
    let db: DatabaseConnection = db().await?;

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
    upsert_hotkeys_in_store()
        .await
        .expect("Failed to upsert hotkeys in store");

    register_hotkeys(register_all);
    get_main_window()
        .emit(ListenEvent::Init.to_string().as_str(), ())
        .expect("Failed to emit init event");

    if let Some(settings_window) =
        get_app().get_webview_window(WebWindow::Settings.to_string().as_str())
    {
        settings_window
            .emit(ListenEvent::Init.to_string().as_str(), ())
            .expect("Failed to emit init event");
    }

    result
}
