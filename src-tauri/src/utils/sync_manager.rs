use crate::prelude::*;
use crate::service::clipboard::{delete_clipboards_db, new_clipboard_event};
use crate::service::settings::update_settings_from_sync;
use crate::service::{
    clipboard::{get_clipboard_uuids_db, get_latest_syncable_cliboards_db, upsert_clipboard_dto},
    sync::{get_sync_provider, get_sync_status, update_sync_status},
};
use crate::service::p2p;
use common::types::sync::{SyncProvider, SyncState};
use std::sync::{
    atomic::{AtomicI64, Ordering},
    Mutex,
};
use sea_orm::prelude::Uuid;
use std::time::Duration;
use tokio::{task::JoinHandle, time};

/// Re-published this often so other devices can show when it was last online.
const DEVICE_PUBLISH_INTERVAL_SECS: i64 = 120;
static LAST_DEVICE_PUBLISH: AtomicI64 = AtomicI64::new(0);
static LAST_PUBLISHED_ADDR: Mutex<Option<serde_json::Value>> = Mutex::new(None);

async fn refresh_devices(provider: &dyn SyncProvider) {
    let device = p2p::this_device();
    let now = chrono::Utc::now().timestamp();
    let addr_changed = *LAST_PUBLISHED_ADDR.lock().expect("addr poisoned") != device.addr;
    if addr_changed || now - LAST_DEVICE_PUBLISH.load(Ordering::Relaxed) >= DEVICE_PUBLISH_INTERVAL_SECS {
        match provider.upsert_device(&device).await {
            Ok(()) => {
                LAST_DEVICE_PUBLISH.store(now, Ordering::Relaxed);
                *LAST_PUBLISHED_ADDR.lock().expect("addr poisoned") = device.addr;
            }
            Err(e) => printlog!("p2p: failed to publish device: {:?}", e),
        }
    }
    if let Ok(devices) = provider.list_devices().await {
        p2p::set_devices(devices);
        tauri::async_runtime::spawn(p2p::probe_devices());
    }
}

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

    /// Returns false when the provider is no longer signed in.
    async fn sync_job() -> Result<bool, Box<dyn std::error::Error>> {
        let provider = get_sync_provider().await;
        if !provider.is_authenticated().await {
            return Ok(false);
        }

        if get_sync_status().email.is_none() {
            let email = provider.account_email().await;
            update_sync_status(|status| status.email = email);
        }

        refresh_devices(provider.as_ref()).await;

        {
            let settings = provider.get_settings().await?;
            update_settings_from_sync(settings).await?;

            let local_clipboards = get_clipboard_uuids_db().await?;
            let mut remote_clipboards = provider.fetch_all_clipboards().await?;

            let deleted_clipboards: Vec<Uuid> = remote_clipboards
                .iter()
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
                upsert_clipboard_dto(clipboard.clone()).await?;
                new_clipboard_event(clipboard);
            }

            let new_local_clipboards = get_latest_syncable_cliboards_db().await?;

            let new_remote_clipboards = provider
                .upload_new_clipboards(&new_local_clipboards, &remote_clipboards)
                .await?;

            remote_clipboards.extend(new_remote_clipboards.into_iter());

            provider.cleanup_old_clipboards(&remote_clipboards).await?;
        }

        Ok(true)
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
            loop {
                // Run the sync job and wait for it to complete
                match Self::sync_job().await {
                    Ok(true) => update_sync_status(|status| {
                        status.state = SyncState::Synced;
                        status.last_synced_at = Some(chrono::Utc::now().timestamp());
                        status.error = None;
                    }),
                    Ok(false) => update_sync_status(|status| {
                        status.state = SyncState::AuthError;
                        status.error = None;
                    }),
                    Err(e) => {
                        printlog!("sync job failed: {:?}", e);
                        let error = e.to_string();
                        update_sync_status(|status| {
                            status.state = SyncState::Error;
                            status.error = Some(error);
                        });
                    }
                }

                time::sleep(interval).await;
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
