use super::clipboard::get_last_clipboard_db;
use crate::prelude::*;
use crate::service::window::get_monitor_scale_factor;
use crate::utils::hotkey_manager::{register_hotkeys, unregister_hotkeys, upsert_hotkeys_in_store};
use common::io::language::get_system_language;
use common::types::enums::{ListenEvent, WebWindow};
use core::future::Future;
use entity::settings::{self, ActiveModel, Model};
use sea_orm::{ActiveModelTrait, EntityTrait};
use tao::connection::db;
use tao::global::get_app;
use tao::global::get_main_window;
use tauri::Emitter;
use tauri::Manager;
use tauri_plugin_autostart::AutoLaunchManager;

pub fn autostart() {
    tauri::async_runtime::spawn(async {
        let settings = get_settings_db().await.expect("Failed to get settings");
        let manager: tauri::State<'_, AutoLaunchManager> = get_app().state::<AutoLaunchManager>();

        // Use the manager as needed
        if settings.startup && !manager.is_enabled().expect("Failed to check auto-launch") {
            manager.enable().expect("Failed to enable auto-launch");
        } else {
            manager.disable().expect("Failed to disable auto-launch");
        }
    });
}

pub async fn get_settings_db() -> Result<Model, DbErr> {
    let db: DatabaseConnection = db().await?;

    let settings = settings::Entity::find_by_id(1)
        .one(&db)
        .await?
        .expect("Settings not found");

    get_app().manage(settings.clone());

    Ok(settings)
}

pub async fn update_settings_db(settings: Model) -> Result<Model, DbErr> {
    let db: DatabaseConnection = db().await?;

    let active_model: ActiveModel = settings.into();

    let updated_settings = settings::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    Ok(updated_settings)
}

pub async fn update_settings_synchronize_db(sync: bool) -> Result<settings::Model, DbErr> {
    let db: DatabaseConnection = db().await?;

    let mut settings = get_settings_db().await?;

    settings.sync = sync;

    let active_model: ActiveModel = settings.into();

    let _ = settings::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    Ok(get_settings_db().await?)
}

pub fn init_settings() {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let last_clipboard = get_last_clipboard_db().await;
            if last_clipboard.is_ok() {
                return;
            }

            let mut settings = get_settings_db().await.expect("Failed to get settings");

            settings.display_scale = get_monitor_scale_factor();
            settings.language = get_system_language().to_string();

            let _ = update_settings_db(settings)
                .await
                .expect("Failed to update settings");
        })
    });
}

pub async fn init_window_settings<T, F>(action: F) -> T
where
    F: Future<Output = T>,
{
    unregister_hotkeys(true);

    let result = action.await;
    upsert_hotkeys_in_store()
        .await
        .expect("Failed to upsert hotkeys in store");

    register_hotkeys(false);
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
