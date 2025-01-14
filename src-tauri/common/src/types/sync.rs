use super::orm_query::FullClipboardDto;
use chrono::NaiveDateTime;
use google_drive3::api::File;
use sea_orm::prelude::Uuid;
use std::{collections::HashMap, time::Instant};

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

pub struct CachedFiles {
    pub files: Vec<File>,
    pub timestamp: Instant,
}
