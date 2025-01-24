use super::clipboard::get_last_clipboard_db;
use super::sync::upsert_settings_sync;
use crate::prelude::*;
use crate::service::window::get_monitor_scale_factor;
use common::io::language::get_system_language;
use common::types::enums::ListenEvent;
use common::types::types::CommandError;
use entity::settings;
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::collections::HashMap;
use std::sync::Mutex;
use tao::connection::db;
use tao::global::get_app;
use tauri::Manager;
use tauri::{Emitter, EventTarget};
use tauri_plugin_autostart::AutoLaunchManager;

pub fn autostart() {
    tauri::async_runtime::spawn(async {
        let settings = get_global_settings();
        let manager: tauri::State<'_, AutoLaunchManager> = get_app().state::<AutoLaunchManager>();

        // Use the manager as needed
        if settings.startup && !manager.is_enabled().expect("Failed to check auto-launch") {
            manager.enable().expect("Failed to enable auto-launch");
        } else {
            manager.disable().expect("Failed to disable auto-launch");
        }
    });
}

pub async fn get_settings_db() -> Result<settings::Model, DbErr> {
    let db: DatabaseConnection = db().await?;

    let settings = settings::Entity::find_by_id(1)
        .one(&db)
        .await?
        .expect("Settings not found");

    set_global_settings(settings.clone());

    Ok(settings)
}

pub async fn update_settings_db(
    settings: settings::Model,
) -> Result<settings::Model, CommandError> {
    let db: DatabaseConnection = db().await?;

    let active_model: settings::ActiveModel = settings.into();

    let settings = settings::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    set_global_settings(settings.clone());

    upsert_settings_sync(&settings).expect("Failed to upsert settings");

    init_settings_window();

    Ok(settings)
}

pub async fn update_settings_synchronize_db(sync: bool) -> Result<settings::Model, DbErr> {
    let db: DatabaseConnection = db().await?;

    let mut settings = get_global_settings();

    settings.sync = sync;

    let active_model: settings::ActiveModel = settings.into();

    let settings = settings::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    set_global_settings(settings.clone());

    init_settings_window();

    Ok(settings)
}

pub fn init_settings() {
    get_app().manage(Mutex::new(settings::Model::default()));

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let last_clipboard = get_last_clipboard_db().await;
            if last_clipboard.is_ok() {
                return;
            }

            let mut settings = get_global_settings();

            settings.display_scale = get_monitor_scale_factor();
            settings.language = get_system_language().to_string();

            let _ = update_settings_db(settings)
                .await
                .expect("Failed to update settings");
        })
    });
}

pub async fn update_settings_from_sync(
    settings: HashMap<String, serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    if settings.is_empty() {
        return Ok(());
    }

    let db: DatabaseConnection = db().await?;
    let current_settings = get_global_settings();

    // Convert current settings to Value to get the schema structure
    let current_value = serde_json::to_value(&current_settings)?;

    // Merge the remote settings with current settings
    // This preserves the structure and types of the current settings
    // while only updating fields that exist in both
    let merged_value = match current_value {
        serde_json::Value::Object(mut map) => {
            for (key, value) in settings {
                // Skip display_scale as it is calculated
                if key == "display_scale" {
                    continue;
                }

                if key == "startup" {
                    continue;
                }

                if map.contains_key(&key) {
                    // Only update if types match or can be converted
                    if let Ok(converted) =
                        serde_json::from_value::<serde_json::Value>(value.clone())
                    {
                        map.insert(key, converted);
                    }
                }
            }
            serde_json::Value::Object(map)
        }
        _ => return Ok(()),
    };

    // Deserialize back into settings model, ignoring errors for individual fields
    if let Ok(new_settings) = serde_json::from_value::<settings::Model>(merged_value) {
        let active_model: settings::ActiveModel = new_settings.into();

        let settings = settings::Entity::update(active_model.reset_all())
            .exec(&db)
            .await?;

        set_global_settings(settings);

        init_settings_window();

        printlog!("(remote) downloaded settings");
    }

    Ok(())
}

pub fn init_settings_window() {
    get_app()
        .emit_to(
            EventTarget::any(),
            ListenEvent::InitSettings.to_string().as_str(),
            (),
        )
        .expect("Failed to emit download progress event");
}

pub fn get_global_settings() -> settings::Model {
    let state = get_app().state::<Mutex<settings::Model>>();
    let locked_settings = state.lock().expect("Failed to lock settings");
    locked_settings.clone()
}

pub fn set_global_settings(settings: settings::Model) {
    let state = get_app().state::<Mutex<settings::Model>>();
    let mut locked_settings = state.lock().expect("Failed to lock settings");
    *locked_settings = settings;
}
