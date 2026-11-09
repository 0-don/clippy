use super::orm_query::FullClipboardDto;
use chrono::NaiveDateTime;
use google_drive3::{hyper_rustls, hyper_util, DriveHub};
use sea_orm::prelude::Uuid;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Clippy {
    pub id: Uuid,
    pub provider_id: String,
    pub star: bool,
    pub encrypted: bool,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncState {
    #[default]
    Disabled,
    Connecting,
    Synced,
    AuthError,
    Error,
}

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct SyncStatus {
    pub state: SyncState,
    pub email: Option<String>,
    pub last_synced_at: Option<i64>,
    pub error: Option<String>,
}

/// One device signed in to the same sync account, published by the device itself.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub os: String,
    pub last_seen: i64,
    /// Serialized iroh EndpointAddr, so peers can dial without DNS discovery.
    #[serde(default)]
    pub addr: Option<serde_json::Value>,
}

#[async_trait::async_trait]
pub trait SyncProvider: Send + Sync {
    async fn account_email(&self) -> Option<String>;

    async fn upsert_device(&self, device: &DeviceInfo) -> Result<(), Box<dyn std::error::Error>>;

    async fn list_devices(&self) -> Result<Vec<DeviceInfo>, Box<dyn std::error::Error>>;

    async fn remove_device(&self, id: &str);

    /// Leaves a command for a device that could not be reached directly.
    async fn push_remote_command(
        &self,
        device_id: &str,
        payload: &str,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Returns and deletes the commands left for this device.
    async fn take_remote_commands(
        &self,
        device_id: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>>;

    async fn fetch_all_clipboards(&self) -> Result<Vec<Clippy>, Box<dyn std::error::Error>>;

    async fn compare_and_fetch_new_clipboards(
        &self,
        local_clipboards: &HashMap<Uuid, (bool, NaiveDateTime)>,
        remote_clipboards: &Vec<Clippy>,
    ) -> Result<Vec<FullClipboardDto>, Box<dyn std::error::Error>>;

    async fn upload_new_clipboards(
        &self,
        new_local_clipboards: &[FullClipboardDto],
        remote_clipboards: &Vec<Clippy>,
    ) -> Result<Vec<Clippy>, Box<dyn std::error::Error>>;

    async fn mark_for_deletion(&self, clippy: &Clippy);

    async fn delete_clipboard(&self, clippy: &Clippy);

    async fn download_by_id(
        &self,
        id: &String,
    ) -> Result<FullClipboardDto, Box<dyn std::error::Error>>;

    async fn upload_clipboard(
        &self,
        clipboard: &FullClipboardDto,
    ) -> Result<Clippy, Box<dyn std::error::Error>>;

    async fn update_clipboard(
        &self,
        local_clipboard: &FullClipboardDto,
        remote_clipboard: &Clippy,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn star_clipboard(&self, clippy: &FullClipboardDto);

    async fn cleanup_old_clipboards(
        &self,
        remote_clipboards: &Vec<Clippy>,
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
