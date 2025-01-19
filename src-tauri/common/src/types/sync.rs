use super::orm_query::FullClipboardDto;
use chrono::NaiveDateTime;
use google_drive3::{hyper_rustls, hyper_util, DriveHub};
use sea_orm::prelude::Uuid;
use std::collections::HashMap;

pub struct ClippyInfo {
    pub id: Uuid,
    pub starred: bool,
    pub timestamp: NaiveDateTime,
    pub provider_id: Option<String>,
}

#[async_trait::async_trait]
pub trait SyncProvider: Send + Sync {
    async fn fetch_all_clipboards(&self) -> Result<Vec<ClippyInfo>, Box<dyn std::error::Error>>;

    async fn fetch_new_clipboards(
        &self,
        local_clipboards: &HashMap<Uuid, NaiveDateTime>,
        remote_clipboards: &Vec<ClippyInfo>,
    ) -> Result<Vec<FullClipboardDto>, Box<dyn std::error::Error>>;

    async fn upload_clipboards(
        &self,
        new_local_clipboards: &[FullClipboardDto],
        remote_clipboards: &mut Vec<ClippyInfo>,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn delete_by_id(&self, id: &String);

    async fn delete_by_uuid(&self, uuid: &Uuid);

    async fn download_by_id(
        &self,
        id: &String,
    ) -> Result<FullClipboardDto, Box<dyn std::error::Error>>;

    async fn cleanup_old_clipboards(
        &self,
        remote_clipboards: &Vec<ClippyInfo>,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn is_authenticated(&self) -> bool;
}

pub struct GoogleDriveProvider {
    pub hub:
        DriveHub<hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>>,
}
