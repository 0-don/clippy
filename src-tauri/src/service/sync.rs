use super::settings::{get_settings_db, update_settings_synchronize_db};
use crate::{
    prelude::*,
    utils::{providers::google_drive::GoogleDriveProviderImpl, sync_manager::SyncManager},
};
use common::types::{enums::SyncProviderType, sync::SyncProvider, types::CommandError};
use std::sync::Arc;
use tao::global::get_app;
use tauri::{Manager, State};
use tokio::sync::Mutex;

pub async fn get_sync_provider() -> State<'static, Arc<dyn SyncProvider>> {
    if let Some(sync_state) = get_app().try_state() {
        return sync_state;
    }

    let provider = match get_settings_db()
        .await
        .expect("settings failed")
        .sync_provider
        .as_str()
    {
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

pub async fn get_sync_manager() -> State<'static, Mutex<SyncManager>> {
    match get_app().try_state() {
        Some(manager) => manager,
        None => {
            let manager = Mutex::new(SyncManager::new().await);
            get_app().manage(manager);
            get_app().state()
        }
    }
}

pub fn init_sync_interval() {
    tauri::async_runtime::spawn(async {
        if let Ok(settings) = get_settings_db().await {
            if settings.sync {
                get_sync_manager().await.lock().await.start().await;
            }
        }
    });
}

pub async fn sync_interval_toggle() -> Result<bool, CommandError> {
    let new_sync_state = !get_settings_db().await?.sync;

    if new_sync_state {
        // Trying to enable sync
        let provider = get_sync_provider().await;
        if !provider.is_authenticated().await {
            update_settings_synchronize_db(false).await?;
            return Err(CommandError::Error("Authentication failed".to_string()));
        }

        // Don't stop existing sync if enabling - just start a new one
        get_sync_manager().await.lock().await.start().await;
        update_settings_synchronize_db(true).await?;
    } else {
        // Trying to disable sync
        get_sync_manager().await.lock().await.stop().await;
        update_settings_synchronize_db(false).await?;
    }

    Ok(new_sync_state)
}
