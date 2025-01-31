use super::settings::{
    get_global_settings, get_settings_db, update_settings_from_sync, update_settings_synchronize_db,
};
use crate::{
    prelude::*,
    tao::global::get_app,
    utils::{providers::google_drive::GoogleDriveProviderImpl, sync_manager::SyncManager},
};
use common::types::{enums::SyncProviderType, sync::SyncProvider, types::CommandError};
use entity::settings;
use std::{collections::HashMap, sync::Arc};
use tauri::{Manager, State};
use tokio::sync::Mutex;

pub async fn upsert_settings_sync(
    settings: &settings::Model,
    block_main_thread: bool,
) -> Result<(), CommandError> {
    if settings.sync {
        let settings_map: HashMap<String, serde_json::Value> =
            serde_json::from_value(serde_json::to_value(settings.clone())?)?;
        if block_main_thread {
            get_sync_provider()
                .await
                .upsert_settings(&settings_map)
                .await
                .expect("Failed to upsert settings");
        } else {
            tauri::async_runtime::spawn(async move {
                get_sync_provider()
                    .await
                    .upsert_settings(&settings_map)
                    .await
                    .expect("Failed to upsert settings");
            });
        }
    }
    Ok(())
}

pub async fn get_sync_provider() -> State<'static, Arc<dyn SyncProvider>> {
    if let Some(sync_state) = get_app().try_state() {
        return sync_state;
    }

    let provider = match get_global_settings().sync_provider.as_str() {
        s if s == SyncProviderType::GoogleDrive.to_string() => Arc::new(
            GoogleDriveProviderImpl::new()
                .await
                .expect("Failed to create Google Drive provider"),
        ),
        _ => panic!("Provider type not implemented"),
    };

    get_app().manage(provider.clone() as Arc<dyn SyncProvider>);
    get_app().state()
}

pub fn get_sync_manager() -> State<'static, Mutex<SyncManager>> {
    match get_app().try_state() {
        Some(manager) => manager,
        None => {
            let manager = Mutex::new(SyncManager::new());
            get_app().manage(manager);
            get_app().state()
        }
    }
}

pub fn setup_sync_interval() {
    tauri::async_runtime::spawn(async {
        // needs to be real db call because of setup race condition in `fn setup_settings()`
        let settings = get_settings_db().await.expect("Failed to get settings");

        if settings.sync {
            get_sync_manager().lock().await.start().await;
        }
    });
}

pub async fn sync_toggle() -> Result<bool, CommandError> {
    let new_sync_state = !get_global_settings().sync;

    if new_sync_state {
        // Trying to enable sync
        let provider = get_sync_provider().await;
        if !provider.is_authenticated().await {
            update_settings_synchronize_db(false).await?;
            return Err(CommandError::Error("Authentication failed".to_string()));
        }

        let remote_settings = provider
            .get_settings()
            .await
            .expect("Failed to get settings");
        update_settings_from_sync(remote_settings)
            .await
            .expect("Failed to update settings");

        let settings = update_settings_synchronize_db(true).await?;

        upsert_settings_sync(&settings, false).await?;

        // Don't stop existing sync if enabling - just start a new one
        get_sync_manager().lock().await.start().await;
    } else {
        // Trying to disable sync
        get_sync_manager().lock().await.stop().await;
        let settings = update_settings_synchronize_db(false).await?;
        upsert_settings_sync(&settings, false).await?;
    }

    Ok(new_sync_state)
}
