use crate::prelude::*;
use crate::service::settings::update_settings_from_sync;
use crate::service::{
    clipboard::{get_clipboard_uuids_db, get_sync_amount_cliboards_db, insert_clipboard_dto},
    sync::get_sync_provider,
};
use std::time::Duration;
use tokio::{task::JoinHandle, time};

pub struct SyncManager {
    job_handle: Option<JoinHandle<()>>,
    is_running: bool,
}

impl SyncManager {
    pub async fn new() -> Self {
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

            let new_clipboards = provider
                .fetch_new_clipboards(&local_clipboards, &remote_clipboards)
                .await?;

            for clipboard in new_clipboards {
                insert_clipboard_dto(clipboard).await?;
            }

            let new_local_clipboards = get_sync_amount_cliboards_db().await?;

            let has_added_new_clipboards = provider
                .upload_new_clipboards(&new_local_clipboards, &mut remote_clipboards)
                .await?;

            if has_added_new_clipboards {
                provider.cleanup_old_clipboards(&remote_clipboards).await?;
            }
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
