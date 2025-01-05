use crate::service::{global::get_app, sync::SyncProvider};
use chrono::{NaiveDateTime, TimeZone, Utc};
use common::{
    constants::{BACKUP_FILE_PREFIX, TOKEN_NAME},
    types::{orm_query::FullClipboardDto, types::CommandError},
};
use entity::settings;
use google_drive3::{api::*, hyper_rustls, hyper_util, yup_oauth2, DriveHub};
use http_body_util::BodyExt;
use migration::async_trait;
use sea_orm::prelude::Uuid;
use std::{
    collections::HashMap,
    io::Cursor,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tauri::Manager;

struct CachedFiles {
    files: Vec<File>,
    timestamp: Instant,
}

pub struct GoogleDriveProvider {
    hub: DriveHub<hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>>,
    cache: Arc<Mutex<Option<CachedFiles>>>,
}

impl GoogleDriveProvider {
    const CACHE_TTL: Duration = Duration::from_secs(20);

    pub async fn new() -> Result<Self, CommandError> {
        let secret = yup_oauth2::ApplicationSecret {
            client_id: std::env::var("GOOGLE_CLIENT_ID")?,
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET")?,
            auth_uri: "https://accounts.google.com/o/oauth2/auth".into(),
            token_uri: "https://accounts.google.com/o/oauth2/token".into(),
            ..Default::default()
        };

        let token_path =
            std::path::Path::new(&crate::service::settings::get_data_path().config_path)
                .join(TOKEN_NAME);
        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .persist_tokens_to_disk(token_path)
        .build()
        .await?;

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()?
                        .https_or_http()
                        .enable_http1()
                        .build(),
                );

        Ok(Self {
            hub: DriveHub::new(client, auth),
            cache: Arc::new(Mutex::new(None)),
        })
    }

    async fn fetch_all_clipboard_files(&self) -> Result<Vec<File>, Box<dyn std::error::Error>> {
        let mut all_files = Vec::new();
        let mut page_token = None;

        while let Ok((_, file_list)) = self
            .hub
            .files()
            .list()
            .q(&format!("name contains '{}'", BACKUP_FILE_PREFIX))
            .spaces("appDataFolder")
            .add_scope(Scope::Appdata.as_ref())
            .page_token(page_token.as_deref().unwrap_or_default())
            .doit()
            .await
        {
            if let Some(files) = file_list.files {
                all_files.extend(files);
            }

            if file_list.next_page_token.is_none() {
                break;
            }
            page_token = file_list.next_page_token;
        }

        println!("Found {} remote clipboard files", all_files.len());

        Ok(all_files)
    }

    async fn get_remote_files(&self) -> Result<Vec<File>, Box<dyn std::error::Error>> {
        // Check cache first
        if let Some(cache) = &*self.cache.lock().unwrap() {
            if cache.timestamp.elapsed() < Self::CACHE_TTL {
                return Ok(cache.files.clone());
            }
        }

        let files = self.fetch_all_clipboard_files().await?;

        // Update cache
        *self.cache.lock().unwrap() = Some(CachedFiles {
            files: files.clone(),
            timestamp: Instant::now(),
        });

        Ok(files)
    }

    async fn cleanup_old_backups(&self) -> Result<(), Box<dyn std::error::Error>> {
        let files = self.get_remote_files().await?;
        let settings = get_app().state::<settings::Model>();

        let mut to_delete: Vec<_> = files
            .iter()
            .filter_map(|f| {
                f.clone()
                    .id
                    .zip(f.clone().name)
                    .zip(Self::parse_clipboard_info(&f.name.as_ref()?))
                    .map(|((id, name), info)| (id, name, info))
            })
            .filter(|(_, _, (_, star, _))| !star)
            .map(|(id, name, (_, _, timestamp))| (id, name, timestamp))
            .collect();

        to_delete.sort_by_key(|(_, _, ts)| *ts);
        let delete_count = to_delete.len().saturating_sub(settings.sync_limit as usize);

        for (id, name, _) in to_delete.into_iter().take(delete_count) {
            println!("Deleting old clipboard: {}", name);
            self.hub
                .files()
                .delete(&id)
                .add_scope(Scope::Appdata.as_ref())
                .doit()
                .await?;
        }

        // Invalidate cache after deletions
        *self.cache.lock().unwrap() = None;
        Ok(())
    }

    fn parse_clipboard_info(filename: &str) -> Option<(Uuid, bool, NaiveDateTime)> {
        let [_, timestamp, star, uuid]: [&str; 4] =
            filename.split('_').collect::<Vec<_>>().try_into().ok()?;
        Some((
            Uuid::parse_str(uuid.trim_end_matches(".json")).ok()?,
            star.parse().ok()?,
            NaiveDateTime::parse_from_str(timestamp, "%Y%m%d%H%M%S").ok()?,
        ))
    }

    async fn download_clipboard(&self, id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let (mut response, _) = self
            .hub
            .files()
            .get(id)
            .param("alt", "media")
            .acknowledge_abuse(true)
            .add_scope(Scope::Appdata.as_ref())
            .doit()
            .await?;

        Ok(String::from_utf8(
            response.body_mut().collect().await?.to_bytes().to_vec(),
        )?)
    }
}

#[async_trait::async_trait]
impl SyncProvider for GoogleDriveProvider {
    async fn fetch_clipboards(
        &self,
        existing_clipboards: &HashMap<Uuid, NaiveDateTime>,
    ) -> Result<Vec<FullClipboardDto>, Box<dyn std::error::Error>> {
        let filelist = self.get_remote_files().await?;
        let mut clipboards = Vec::new();

        for file in filelist {
            if let Some(id) = file.id.as_ref() {
                if let Some((uuid, _star, timestamp)) =
                    Self::parse_clipboard_info(&file.name.as_ref().expect("No name"))
                {
                    if let Some(existing_timestamp) = existing_clipboards.get(&uuid) {
                        if existing_timestamp >= &timestamp {
                            continue;
                        }
                    }

                    println!("Downloading clipboard: {:?}", file.name);
                    match self.download_clipboard(id).await {
                        Ok(content) => {
                            if let Ok(clipboard) = serde_json::from_str(&content) {
                                clipboards.push(clipboard);
                            }
                        }
                        Err(e) => println!("Error downloading clipboard {:?}: {}", file.name, e),
                    }
                }
            }
        }

        Ok(clipboards)
    }

    async fn upload_clipboards(
        &self,
        local_clipboards: &[FullClipboardDto],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let remote_files = self.get_remote_files().await?;
        let remote_map: HashMap<Uuid, (String, NaiveDateTime)> = remote_files
            .iter()
            .filter_map(|f| {
                Self::parse_clipboard_info(&f.name.as_ref()?).and_then(
                    |(uuid, _star, timestamp)| {
                        f.id.as_ref().map(|id| (uuid, (id.clone(), timestamp)))
                    },
                )
            })
            .collect();

        for clipboard in local_clipboards {
            if let Some((remote_id, remote_timestamp)) = remote_map.get(&clipboard.clipboard.id) {
                if remote_timestamp >= &clipboard.clipboard.created_date {
                    continue;
                }
                self.hub.files().delete(remote_id).doit().await?;
                // Invalidate cache after deletion
                *self.cache.lock().unwrap() = None;
            }

            let file_name = format!(
                "{}_{}_{}_{}.json",
                BACKUP_FILE_PREFIX,
                clipboard.clipboard.created_date.format("%Y%m%d%H%M%S"),
                clipboard.clipboard.star,
                clipboard.clipboard.id
            );

            println!("Uploading clipboard: {}", file_name);

            let file = File {
                name: Some(file_name),
                mime_type: Some("application/json".into()),
                created_time: Some(Utc.from_utc_datetime(&clipboard.clipboard.created_date)),
                parents: Some(vec!["appDataFolder".into()]),
                ..Default::default()
            };

            self.hub
                .files()
                .create(file)
                .add_scope(Scope::Appdata.as_ref())
                .upload(
                    Cursor::new(serde_json::to_string(&clipboard)?),
                    "application/json".parse()?,
                )
                .await?;
        }

        self.cleanup_old_backups().await
    }

    async fn delete_by_id(&self, clipboard_uuid: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let files = self.fetch_all_clipboard_files().await?;

        let file_id = files.into_iter().find_map(|file| {
            if let Some((uuid, _, _)) = Self::parse_clipboard_info(&file.name.as_ref()?) {
                if &uuid == clipboard_uuid {
                    file.id
                } else {
                    None
                }
            } else {
                None
            }
        });

        if let Some(file_id) = file_id {
            self.hub
                .files()
                .delete(&file_id)
                .add_scope(Scope::Appdata.as_ref())
                .doit()
                .await?;
            Ok(())
        } else {
            Err("No file found with the specified UUID".into())
        }
    }

    async fn is_authenticated(&self) -> bool {
        match self.hub.auth.get_token(&[Scope::Appdata.as_ref()]).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
