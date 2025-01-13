use super::settings::{get_settings_db, update_settings_synchronize_db};
use crate::{
    prelude::*,
    service::clipboard::{
        get_clipboard_uuids_db, get_sync_amount_cliboards_db, insert_clipboard_dto,
    },
    utils::providers::google_drive::GoogleDriveProvider,
};
use chrono::NaiveDateTime;
use common::types::{orm_query::FullClipboardDto, types::CommandError};
use sea_orm::prelude::Uuid;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::mpsc::{self, Sender};
use tokio::time::interval;

static SYNC_MANAGER: Mutex<Option<SyncManager>> = Mutex::new(None);

#[async_trait::async_trait]
pub trait SyncProvider: Send + Sync {
    async fn fetch_clipboards(
        &self,
        existing_clipboards: &HashMap<Uuid, NaiveDateTime>,
    ) -> Result<Vec<FullClipboardDto>, Box<dyn std::error::Error>>;

    async fn upload_clipboards(
        &self,
        clipboards: &[FullClipboardDto],
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn delete_by_id(&self, id: &Uuid) -> Result<(), Box<dyn std::error::Error>>;

    async fn is_authenticated(&self) -> bool;
}

pub struct SyncManager {
    provider: Arc<dyn SyncProvider>,
    interval_secs: u64,
    shutdown_tx: Option<Sender<()>>,
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
                    _ = rx.recv() => {
                        printlog!("Shutting down sync manager");
                        break;
                    }
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

pub fn init_sync_interval() {
    tauri::async_runtime::spawn(async {
        // Get settings to check if sync is enabled
        if let Ok(settings) = get_settings_db().await {
            if settings.sync {
                // Only start sync if it's enabled in settings
                let provider = Arc::new(
                    GoogleDriveProvider::new()
                        .await
                        .expect("Failed to initialize sync provider"),
                );

                let mut manager = SyncManager::new(provider, 30);
                manager.start().await;

                // Store the manager
                let mut sync_manager = SYNC_MANAGER.lock().expect("Failed to lock sync manager");
                *sync_manager = Some(manager);
            }
        }
    });
}

pub async fn sync_interval_toggle() -> Result<bool, CommandError> {
    let settings = get_settings_db().await?;
    let new_sync_state = !settings.sync;

    update_settings_synchronize_db(new_sync_state).await?;

    if new_sync_state {
        let provider = Arc::new(
            GoogleDriveProvider::new()
                .await
                .expect("Failed to initialize sync provider"),
        );

        let mut manager = SyncManager::new(provider, 30);
        manager.start().await;

        // Create a new scope for the MutexGuard
        {
            let mut sync_manager = SYNC_MANAGER.lock().expect("Failed to lock sync manager");
            *sync_manager = Some(manager);
        }
    } else {
        // Create a new scope for the MutexGuard
        let manager = {
            let mut sync_manager = SYNC_MANAGER.lock().expect("Failed to lock sync manager");
            sync_manager.take()
        };

        if let Some(mut manager) = manager {
            manager.stop().await;
        }
    }

    Ok(new_sync_state)
}
