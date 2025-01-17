use super::orm_query::FullClipboardDto;
use chrono::NaiveDateTime;
use google_drive3::{hyper_rustls, hyper_util, DriveHub};
use sea_orm::prelude::Uuid;
use std::collections::HashMap;

pub struct ClippyInfo {
    pub id: Uuid,
    pub starred: bool,
    pub timestamp: NaiveDateTime,
    pub provider_id: String,
}

#[async_trait::async_trait]
pub trait SyncProvider: Send + Sync {
    async fn fetch_all_clipboards(
        &self,
    ) -> Result<Vec<ClippyInfo>, Box<dyn std::error::Error>>;

    async fn fetch_new_clipboards(
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

pub struct GoogleDriveProvider {
    pub hub:
        DriveHub<hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>>,
}
