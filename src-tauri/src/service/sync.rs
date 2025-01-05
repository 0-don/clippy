use crate::{
    prelude::*,
    service::clipboard::{
        get_clipboard_uuids_db, get_sync_amount_cliboards_db, insert_clipboard_dto,
    },
    utils::providers::google_drive::GoogleDriveProvider,
};
use chrono::NaiveDateTime;
use common::types::orm_query::FullClipboardDto;
use sea_orm::prelude::Uuid;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::time::interval;

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
}

impl SyncManager {
    pub fn new(provider: Arc<dyn SyncProvider>, interval_secs: u64) -> Self {
        Self {
            provider,
            interval_secs,
        }
    }

    pub async fn start(&self) {
        let mut interval = interval(Duration::from_secs(self.interval_secs));

        // Initial delay
        tokio::time::sleep(Duration::from_secs(3)).await;

        loop {
            interval.tick().await;
            printlog!("Syncing clipboards...");

            if self.provider.is_authenticated().await {
                if let Err(e) = self.sync_clipboards().await {
                    printlog!("Sync error: {}", e);
                }
            } else {
                printlog!("Not authenticated for sync");
            }
        }
    }

    async fn sync_clipboards(&self) -> Result<(), Box<dyn std::error::Error>> {
        let existing_clipboards = get_clipboard_uuids_db().await?;

        let remote_clipboards = self.provider.fetch_clipboards(&existing_clipboards).await?;
        printlog!("New {} remote clipboards", remote_clipboards.len());

        for clipboard in remote_clipboards {
            if let Err(e) = insert_clipboard_dto(clipboard).await {
                printlog!("Error inserting remote clipboard: {}", e);
            }
        }

        let local_clipboards = get_sync_amount_cliboards_db().await?;
        // Upload to provider
        self.provider.upload_clipboards(&local_clipboards).await?;

        Ok(())
    }
}

pub fn init_sync_watch() {
    tauri::async_runtime::spawn(async {
        let provider = Arc::new(
            GoogleDriveProvider::new()
                .await
                .expect("Failed to initialize sync provider"),
        );

        let sync_manager = SyncManager::new(provider, 30);
        sync_manager.start().await;
    });
}
