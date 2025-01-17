use crate::prelude::*;
use crate::service::{
    clipboard::{get_clipboard_uuids_db, get_sync_amount_cliboards_db, insert_clipboard_dto},
    sync::get_sync_provider,
};
use sea_orm::prelude::Uuid;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

pub struct SyncManager {
    pub scheduler: JobScheduler,
    pub job_id: Option<Uuid>,
}

impl SyncManager {
    pub async fn new() -> Self {
        Self {
            scheduler: JobScheduler::new()
                .await
                .expect("Failed to create scheduler"),
            job_id: None,
        }
    }

    async fn sync_job() {
        let provider = get_sync_provider().await;
        if provider.is_authenticated().await {
            let existing_clipboards = get_clipboard_uuids_db()
                .await
                .expect("Failed to get clipboard UUIDs");

            let remote_clipboards = provider
                .fetch_new_clipboards(&existing_clipboards)
                .await
                .expect("Failed to fetch remote clipboards");

            printlog!("(remote) {} new clipboards", remote_clipboards.len());

            for clipboard in remote_clipboards {
                if let Err(e) = insert_clipboard_dto(clipboard).await {
                    printlog!("Error inserting remote clipboard: {}", e);
                }
            }

            let local_clipboards = get_sync_amount_cliboards_db()
                .await
                .expect("Failed to get local clipboards");

            if let Err(e) = provider.upload_clipboards(&local_clipboards).await {
                printlog!("Error uploading clipboards: {}", e);
            }
        }
    }

    pub async fn start(&mut self) {
        if self.job_id.is_some() {
            printlog!("Sync already running");
            return;
        }
        let interval_secs = if cfg!(debug_assertions) { 5 } else { 30 };

        let job = Job::new_repeated_async(Duration::from_secs(interval_secs), move |_, _| {
            Box::pin(async move {
                Self::sync_job().await;
            })
        })
        .expect("Failed to create job");

        let job_id = self
            .scheduler
            .add(job)
            .await
            .expect("Failed to add job to scheduler");
        self.job_id = Some(job_id);

        if let Err(err) = self.scheduler.start().await {
            printlog!("Error starting scheduler: {}", err);
        }
        printlog!("Sync loop started");
    }

    pub async fn stop(&mut self) {
        if let Some(job_id) = self.job_id {
            if let Err(err) = self.scheduler.remove(&job_id).await {
                printlog!("Error removing job: {}", err);
            }
            self.job_id = None;
            printlog!("Sync loop stopped");
        } else {
            printlog!("Sync loop not running");
        }
    }
}
