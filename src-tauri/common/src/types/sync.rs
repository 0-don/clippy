use super::orm_query::FullClipboardDto;
use chrono::NaiveDateTime;
use google_drive3::{hyper_rustls, hyper_util, DriveHub};
use sea_orm::prelude::Uuid;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ClippyInfo {
    pub id: Uuid,
    pub provider_id: String,
    pub starred: bool,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[async_trait::async_trait]
pub trait SyncProvider: Send + Sync {
    async fn fetch_all_clipboards(&self) -> Result<Vec<ClippyInfo>, Box<dyn std::error::Error>>;

    async fn compare_and_fetch_new_clipboards(
        &self,
        local_clipboards: &HashMap<Uuid, NaiveDateTime>,
        remote_clipboards: &Vec<ClippyInfo>,
    ) -> Result<Vec<FullClipboardDto>, Box<dyn std::error::Error>>;

    async fn upload_new_clipboards(
        &self,
        new_local_clipboards: &[FullClipboardDto],
        remote_clipboards: &Vec<ClippyInfo>,
    ) -> Result<Vec<ClippyInfo>, Box<dyn std::error::Error>>;

    async fn mark_for_deletion(&self, clippy: &ClippyInfo);

    async fn delete_clipboard(&self, clippy: &ClippyInfo);

    async fn download_by_id(
        &self,
        id: &String,
    ) -> Result<FullClipboardDto, Box<dyn std::error::Error>>;

    async fn upload_clipboard(
        &self,
        clipboard: &FullClipboardDto,
    ) -> Result<ClippyInfo, Box<dyn std::error::Error>>;

    async fn star_clipboard(&self, clippy: &FullClipboardDto);

    async fn cleanup_old_clipboards(
        &self,
        remote_clipboards: &Vec<ClippyInfo>,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn upsert_settings(
        &self,
        settings: &HashMap<String, Value>,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn get_settings(&self) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>>;

    async fn is_authenticated(&self) -> bool;
}

pub struct GoogleDriveProvider {
    pub hub:
        DriveHub<hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>>,
}
