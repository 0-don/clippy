use crate::prelude::*;
use crate::service::clipboard::delete_clipboards_db;
use crate::service::settings::update_settings_from_sync;
use crate::service::{
    clipboard::{get_clipboard_uuids_db, get_latest_syncable_cliboards_db, insert_clipboard_dto},
    sync::get_sync_provider,
};
use sea_orm::prelude::Uuid;
use std::time::Duration;
use tokio::{task::JoinHandle, time};

pub struct SyncManager {
    job_handle: Option<JoinHandle<()>>,
    is_running: bool,
}

impl SyncManager {
    pub fn new() -> Self {
        Self {
            job_handle: None,
            is_running: false,
        }
    }

    async fn sync_job() -> Result<(), Box<dyn std::error::Error>> {
        let provider = get_sync_provider().await;
        if provider.is_authenticated().await {
            let settings = provider.get_settings().await?;
            update_settings_from_sync(settings).await?;

            let local_clipboards = get_clipboard_uuids_db().await?;
            let mut remote_clipboards = provider.fetch_all_clipboards().await?;

            let deleted_clipboards: Vec<Uuid> = remote_clipboards
                .clone()
                .into_iter()
                .filter(|clipboard| clipboard.deleted_at.is_some())
                .map(|clipboard| clipboard.id)
                .collect();

            delete_clipboards_db(deleted_clipboards, None)
                .await
                .expect("Error deleting clipboards");

            let new_clipboards = provider
                .compare_and_fetch_new_clipboards(&local_clipboards, &remote_clipboards)
                .await?;

            for clipboard in new_clipboards {
                insert_clipboard_dto(clipboard).await?;
            }

            let new_local_clipboards = get_latest_syncable_cliboards_db().await?;

            let new_remote_clipboards = provider
                .upload_new_clipboards(&new_local_clipboards, &remote_clipboards)
                .await?;

            remote_clipboards.extend(new_remote_clipboards.into_iter());

            provider.cleanup_old_clipboards(&remote_clipboards).await?;
        }

        Ok(())
    }

    pub async fn start(&mut self) {
        if self.is_running {
            printlog!("sync already running");
            return;
        }

        let interval = if cfg!(debug_assertions) {
            Duration::from_secs(10)
        } else {
            Duration::from_secs(30)
        };

        // Create a new sync task
        let handle = tokio::spawn(async move {
            let mut interval = time::interval(interval);
            loop {
                interval.tick().await;
                if let Err(e) = Self::sync_job().await {
                    printlog!("sync job failed: {:?}", e);
                }
            }
        });

        self.job_handle = Some(handle);
        self.is_running = true;
        printlog!("sync loop started");
    }

    pub async fn stop(&mut self) {
        if let Some(handle) = self.job_handle.take() {
            handle.abort();
            self.is_running = false;
            printlog!("sync loop stopped");
        } else {
            printlog!("sync loop not running");
        }
    }
}
