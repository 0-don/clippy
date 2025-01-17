use super::settings::{get_settings_db, init_window_settings, update_settings_synchronize_db};
use crate::{
    prelude::*,
    utils::{
        providers::google_drive::GoogleDriveProvider,
        sync_manager::{SyncManager, SYNC_MANAGER},
    },
};
use common::types::{enums::SyncProviderType, sync::SyncProvider, types::CommandError};
use entity::settings;
use std::sync::Arc;
use tao::global::get_app;
use tauri::Manager;

pub async fn get_sync_provider() -> Result<Arc<dyn SyncProvider>, CommandError> {
    match get_app().state::<settings::Model>().sync_provider.as_str() {
        s if s == SyncProviderType::GoogleDrive.to_string() => {
            Ok(Arc::new(GoogleDriveProvider::new().await?))
        }
        _ => Err(CommandError::Error("Unsupported sync provider".into())),
    }
}

pub fn init_sync_interval() {
    tauri::async_runtime::spawn(async {
        if let Ok(settings) = get_settings_db().await {
            if settings.sync {
                if let Ok(provider) = get_sync_provider().await {
                    let mut manager = SyncManager::new(provider, 30);
                    manager.start().await;
                    *SYNC_MANAGER.lock().unwrap() = Some(manager);
                }
            }
        }
    });
}

pub async fn sync_interval_toggle() -> Result<bool, CommandError> {
    let new_sync_state = !get_settings_db().await?.sync;

    if new_sync_state {
        let provider = Arc::new(GoogleDriveProvider::new().await?);
        let mut manager = SyncManager::new(provider, 30);
        manager.start().await;

        {
            let mut sync_manager = SYNC_MANAGER.lock().unwrap();
            *sync_manager = Some(manager);
        }
    } else {
        init_window_settings(async {
            update_settings_synchronize_db(false)
                .await
                .expect("Failed to update settings");
        })
        .await;
        let manager = SYNC_MANAGER.lock().unwrap().take();
        if let Some(mut manager) = manager {
            manager.stop().await;
        }
    }

    Ok(new_sync_state)
}
