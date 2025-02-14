use super::cipher::{init_password_lock_event, is_encryption_key_set};
use super::clipboard::get_last_clipboard_db;
use super::decrypt::decrypt_all_clipboards;
use super::sync::upsert_settings_sync;
use crate::prelude::*;
use crate::service::window::get_monitor_scale_factor;
use crate::tao::connection::db;
use crate::tao::global::get_app;
use common::io::language::get_system_language;
use common::types::enums::{ListenEvent, PasswordAction};
use common::types::types::{CommandError, TextMatcher};
use entity::settings;
use sea_orm::{ActiveModelTrait, EntityTrait};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::Manager;
use tauri::{Emitter, EventTarget};
use tauri_plugin_autostart::AutoLaunchManager;

pub fn autostart() {
    tauri::async_runtime::spawn(async {
        let settings = get_global_settings();
        let manager = get_app().state::<AutoLaunchManager>();

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

    upsert_settings_sync(&settings, false).await?;

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

pub fn setup_settings() {
    get_app().manage(Mutex::new(settings::Model::default()));

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

pub async fn update_settings_text_matchers(
    text_matchers: Vec<TextMatcher>,
) -> Result<Vec<TextMatcher>, CommandError> {
    let mut settings = get_global_settings();

    settings.text_matchers = json!(text_matchers);

    let active_model: settings::ActiveModel = settings.into();

    let settings = settings::Entity::update(active_model.reset_all())
        .exec(&db().await?)
        .await?;

    set_global_settings(settings.clone());

    init_settings_window();

    upsert_settings_sync(&settings, false).await?;

    Ok(text_matchers)
}

pub async fn update_settings_from_sync(
    remote_settings: HashMap<String, serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Return early if no settings to process
    if remote_settings.is_empty() {
        return Ok(());
    }

    let db: DatabaseConnection = db().await?;
    let current_settings = get_global_settings();

    // Handle encryption state changes
    let remote_encryption = remote_settings
        .get("encryption")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let mut remote_settings = remote_settings;

    // Skip encryption as it is handled separately
    remote_settings.remove("encryption");
    // Skip display_scale as it is calculated on first time setup
    remote_settings.remove("display_scale");
    // Skip startup as it users choice
    remote_settings.remove("startup");

    let local_encryption = current_settings.encryption;

    match (local_encryption, remote_encryption, is_encryption_key_set()) {
        // Local unencrypted -> Remote encrypted
        (false, true, false) => {
            printlog!("Local unencrypted -> Remote encrypted");
            remote_settings.insert("encryption".to_string(), serde_json::Value::Bool(true));
            init_password_lock_event(PasswordAction::Encrypt);
        }
        // Local encrypted -> Remote unencrypted, no key
        (true, false, false) => {
            printlog!("Local encrypted -> Remote unencrypted, no key");
            init_password_lock_event(PasswordAction::SyncDecrypt);
        }
        // Local encrypted -> Remote unencrypted, has key
        (true, false, true) => {
            printlog!("Local encrypted -> Remote unencrypted, has key");

            remote_settings.insert("encryption".to_string(), serde_json::Value::Bool(false));

            decrypt_all_clipboards()
                .await
                .expect("Failed to decrypt clipboards");
        }
        _ => {}
    }

    // Convert current settings to Value to get the schema structure
    let current_value = serde_json::to_value(&current_settings)?;

    // Merge the remote settings with current settings
    // This preserves the structure and types of the current settings
    // while only updating fields that exist in both
    let merged_value = match current_value {
        serde_json::Value::Object(mut map) => {
            for (key, value) in remote_settings {
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

        // Update global settings
        set_global_settings(settings);

        // Notify UI of settings change
        init_settings_window();

        printlog!("(remote) applied settings");
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
