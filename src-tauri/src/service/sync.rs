use super::settings::{get_settings_db, update_settings_synchronize_db};
use crate::{
    prelude::*, service::clipboard::*, utils::providers::google_drive::GoogleDriveProvider,
};
use common::types::{enums::SyncProviderType, sync::SyncProvider, types::CommandError};
use entity::settings;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tao::global::get_app;
use tauri::Manager;
use tokio::{sync::mpsc, time::interval};

pub static SYNC_MANAGER: Mutex<Option<SyncManager>> = Mutex::new(None);

pub struct SyncManager {
    provider: Arc<dyn SyncProvider>,
    interval_secs: u64,
    shutdown_tx: Option<mpsc::Sender<()>>,
}

impl SyncManager {
    pub fn new(provider: Arc<dyn SyncProvider>, interval_secs: u64) -> Self {
        Self {
            provider,
            interval_secs,
            shutdown_tx: None,
        }
    }

    pub async fn start(&mut self) {
        let (tx, mut rx) = mpsc::channel(1);
        self.shutdown_tx = Some(tx);
        let provider = self.provider.clone();
        let interval_secs = self.interval_secs;
        tauri::async_runtime::spawn(async move {
            let mut interval = interval(Duration::from_secs(interval_secs));
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        printlog!("Syncing clipboards...");
                        if provider.is_authenticated().await {
                            let existing_clipboards = get_clipboard_uuids_db().await
                                .expect("Failed to get clipboard UUIDs");

                            let remote_clipboards = provider.fetch_clipboards(&existing_clipboards).await
                                .expect("Failed to fetch remote clipboards");

                            printlog!("New {} remote clipboards", remote_clipboards.len());

                            for clipboard in remote_clipboards {
                                if let Err(e) = insert_clipboard_dto(clipboard).await {
                                    printlog!("Error inserting remote clipboard: {}", e);
                                }
                            }

                            let local_clipboards = get_sync_amount_cliboards_db().await
                                .expect("Failed to get local clipboards");

                            if let Err(e) = provider.upload_clipboards(&local_clipboards).await {
                                printlog!("Error uploading clipboards: {}", e);
                            }
                        } else {
                            printlog!("Not authenticated for sync");
                        }
                    }
                    _ = rx.recv() => break
                }
            }
        });
    }

    pub async fn stop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(()).await;
        }
    }
}

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
    update_settings_synchronize_db(new_sync_state).await?;

    if new_sync_state {
        let provider = Arc::new(GoogleDriveProvider::new().await?);
        let mut manager = SyncManager::new(provider, 30);
        manager.start().await;

        {
            let mut sync_manager = SYNC_MANAGER.lock().unwrap();
            *sync_manager = Some(manager);
        }
    } else {
        let manager = SYNC_MANAGER.lock().unwrap().take();
        if let Some(mut manager) = manager {
            manager.stop().await;
        }
    }

    Ok(new_sync_state)
}
