use super::parse_clipboard_info;
use crate::prelude::*;
use crate::{
    service::settings::{get_global_settings, update_settings_synchronize_db},
    tao::{config::get_data_path, global::get_app},
    utils::providers::{create_clipboard_filename, uuid_to_datetime},
};
use chrono::{NaiveDateTime, TimeZone, Utc};
use common::{
    constants::{BACKUP_FILE_PREFIX, BACKUP_SETTINGS_PREFIX, TOKEN_NAME},
    printlog,
    types::{
        enums::ListenEvent,
        orm_query::FullClipboardDto,
        sync::{Clippy, GoogleDriveProvider, SyncProvider},
        types::{CommandError, Progress},
    },
};
use google_drive3::{
    api::*,
    hyper_rustls, hyper_util,
    yup_oauth2::{self, authenticator_delegate::InstalledFlowDelegate},
    DriveHub,
};
use http_body_util::BodyExt;
use migration::async_trait;
use sea_orm::prelude::Uuid;
use serde_json::Value;
use std::{collections::HashMap, future::Future, io::Cursor, pin::Pin};
use tauri::{Emitter, EventTarget, Manager};
use tauri_plugin_clipboard::Clipboard;
use tauri_plugin_opener::OpenerExt;

pub struct BrowserUrlOpenFlowDelegate;

#[async_trait::async_trait]
impl InstalledFlowDelegate for BrowserUrlOpenFlowDelegate {
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        _need_code: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        Box::pin(async move {
            get_app()
                .state::<Clipboard>()
                .write_text(url.to_string())
                .expect("Failed to write URL to clipboard");
            get_app()
                .opener()
                .open_url(url, None::<String>)
                .expect("Failed to open URL");
            Ok(String::new())
        })
    }
}

pub struct GoogleDriveProviderImpl(GoogleDriveProvider);

impl GoogleDriveProviderImpl {
    pub async fn new() -> Result<Self, CommandError> {
        let config = get_app().config();

        let (client_id, client_secret) = match (
            std::env::var("TAURI_GOOGLE_CLIENT_ID"),
            std::env::var("TAURI_GOOGLE_CLIENT_SECRET"),
        ) {
            (Ok(id), Ok(secret)) => (id, secret),
            _ => {
                let plugins = config
                    .plugins
                    .0
                    .get("oauth")
                    .and_then(|o| o.get("google"))
                    .ok_or_else(|| CommandError::new("Missing Google OAuth configuration"))?;

                (
                    plugins
                        .get("clientId")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| CommandError::new("Missing Google client ID"))?
                        .to_string(),
                    plugins
                        .get("clientSecret")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| CommandError::new("Missing Google client secret"))?
                        .to_string(),
                )
            }
        };

        let secret = yup_oauth2::ApplicationSecret {
            client_id,
            client_secret,
            auth_uri: "https://accounts.google.com/o/oauth2/auth".into(),
            token_uri: "https://accounts.google.com/o/oauth2/token".into(),
            ..Default::default()
        };

        let token_path = std::path::Path::new(&get_data_path().config_path).join(TOKEN_NAME);

        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .flow_delegate(Box::new(BrowserUrlOpenFlowDelegate))
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

        let provider = GoogleDriveProvider {
            hub: DriveHub::new(client, auth),
        };

        let impl_provider = Self(provider); // Changed: Use tuple struct initialization

        if impl_provider.is_authenticated().await {
            printlog!("authenticated with Google Drive");
            update_settings_synchronize_db(true)
                .await
                .expect("Failed to update settings");
        }

        Ok(impl_provider)
    }

    async fn fetch_all_clipboard_files(&self) -> Result<Vec<File>, Box<dyn std::error::Error>> {
        let mut all_files = Vec::new();
        let mut page_token = None;

        while let Ok((_, file_list)) = self
            .0
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

        Ok(all_files)
    }

    async fn find_settings_file(&self) -> Result<Option<File>, Box<dyn std::error::Error>> {
        let (_, file_list) = self
            .0
            .hub
            .files()
            .list()
            .q(&format!("name contains '{}'", BACKUP_SETTINGS_PREFIX))
            .spaces("appDataFolder")
            .add_scope(Scope::Appdata.as_ref())
            .doit()
            .await?;

        Ok(file_list.files.and_then(|files| files.into_iter().next()))
    }
}

#[async_trait::async_trait]
impl SyncProvider for GoogleDriveProviderImpl {
    async fn fetch_all_clipboards(&self) -> Result<Vec<Clippy>, Box<dyn std::error::Error>> {
        let filelist = self.fetch_all_clipboard_files().await?;
        let mut clipboards = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        for file in filelist {
            if let Some(remote) =
                parse_clipboard_info(&file.name.expect("No name"), &file.id.expect("No id"))
            {
                // Only add if we haven't seen this ID before
                if seen_ids.insert(remote.id) {
                    clipboards.push(remote);
                } else {
                    printlog!(
                        "Found duplicate clipboard: {} from {}, deleting duplicate",
                        remote.id,
                        uuid_to_datetime(&remote.id)
                    );
                    // Delete the duplicate file
                    self.delete_clipboard(&remote).await;
                }
            }
        }

        // Sort by created_at
        clipboards.sort_by(|a, b| b.id.cmp(&a.id));

        if !clipboards.is_empty() {
            let newest = &clipboards[0];
            let oldest = &clipboards[clipboards.len() - 1];
            printlog!(
                "(remote) found {} clipboards from {} to {}",
                clipboards.len(),
                uuid_to_datetime(&oldest.id),
                uuid_to_datetime(&newest.id)
            );
        } else {
            printlog!("(remote) found no clipboards");
        }

        Ok(clipboards)
    }

    async fn compare_and_fetch_new_clipboards(
        &self,
        local_clipboards: &HashMap<Uuid, (bool, NaiveDateTime)>,
        remote_clipboards: &Vec<Clippy>,
    ) -> Result<Vec<FullClipboardDto>, Box<dyn std::error::Error>> {
        let mut new_clipboards = Vec::new();

        let total = remote_clipboards.len();
        for (index, file) in remote_clipboards.iter().enumerate() {
            // Skip if the clipboard is marked for deletion
            if file.deleted_at.is_some() {
                continue;
            }

            if let Some((local_star, _local_timestamp)) = local_clipboards.get(&file.id) {
                // star status is different
                if local_star == &file.star {
                    continue;
                }
            }

            printlog!(
                "downloading clipboard: {} from {} star: {} encrypted: {}",
                file.id,
                uuid_to_datetime(&file.id),
                file.star,
                file.encrypted
            );

            get_app().emit_to(
                EventTarget::any(),
                ListenEvent::Progress.to_string().as_str(),
                Progress {
                    label: "SETTINGS.ENCRYPT.DOWNLOADING_REMOTE_CLIPBOARDS".to_string(),
                    total,
                    current: index + 1,
                },
            )?;

            new_clipboards.push(self.download_by_id(&file.provider_id).await?);
        }

        Ok(new_clipboards)
    }

    async fn upload_new_clipboards(
        &self,
        new_local_clipboards: &[FullClipboardDto],
        remote_clipboards: &Vec<Clippy>,
    ) -> Result<Vec<Clippy>, Box<dyn std::error::Error>> {
        let mut new_clipboards = Vec::new();

        // Skip clipboards that were recently deleted
        let recently_deleted: Vec<_> = remote_clipboards
            .iter()
            .filter(|c| c.deleted_at.is_some())
            .map(|c| c.id)
            .collect();

        for clipboard in new_local_clipboards {
            if remote_clipboards
                .iter()
                .any(|clip| clip.id == clipboard.clipboard.id)
                || recently_deleted.contains(&clipboard.clipboard.id)
            {
                continue;
            }
            new_clipboards.push(self.upload_clipboard(clipboard).await?);
        }

        Ok(new_clipboards)
    }

    async fn mark_for_deletion(&self, clippy: &Clippy) {
        let new_name = create_clipboard_filename(
            &clippy.id,
            &clippy.star,
            &clippy.encrypted,
            &clippy.created_at,
            Some(Utc::now().naive_utc()),
        );

        let file = google_drive3::api::File {
            name: Some(new_name),
            ..Default::default()
        };

        self.0
            .hub
            .files()
            .update(file, &clippy.provider_id)
            .add_scope(Scope::Appdata.as_ref())
            .doit_without_upload()
            .await
            .expect("Failed to rename file");

        printlog!("(remote) marked clipboard for deletion: {}", clippy.id);
    }

    async fn delete_clipboard(&self, clippy: &Clippy) {
        self.0
            .hub
            .files()
            .delete(&clippy.provider_id)
            .add_scope(Scope::Appdata.as_ref())
            .doit()
            .await
            .ok();
    }

    async fn download_by_id(
        &self,
        id: &String,
    ) -> Result<FullClipboardDto, Box<dyn std::error::Error>> {
        let (mut response, _) = self
            .0
            .hub
            .files()
            .get(id)
            .param("alt", "media")
            .acknowledge_abuse(true)
            .add_scope(Scope::Appdata.as_ref())
            .doit()
            .await?;

        let content = String::from_utf8(response.body_mut().collect().await?.to_bytes().to_vec())?;
        Ok(serde_json::from_str(&content)?)
    }

    async fn cleanup_old_clipboards(
        &self,
        remote_clipboards: &Vec<Clippy>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sync_limit = get_global_settings().sync_limit as usize;

        // Get all non-starred clipboards
        let mut all_clipboards: Vec<_> =
            remote_clipboards.iter().filter(|clip| !clip.star).collect();

        // Sort by creation date
        all_clipboards.sort_by(|a, b| a.id.cmp(&b.id));

        // Find all marked-for-deletion indices
        let marked_indices: Vec<usize> = all_clipboards
            .iter()
            .enumerate()
            .filter(|(_, clip)| clip.deleted_at.is_some())
            .map(|(idx, _)| idx)
            .collect();

        // Only delete if we have clipboards beyond the sync limit
        if all_clipboards.len() <= sync_limit {
            return Ok(());
        }

        if let Some(&last_marked_idx) = marked_indices.last() {
            // Calculate how many clipboards we need to delete

            let total_to_delete = all_clipboards.len() - sync_limit + marked_indices.len();

            printlog!(
                "total clipboards: {}, marked clipboards: {}, total to delete: {} (last marked: {})",
                all_clipboards.len(),
                marked_indices.len(),
                total_to_delete,
                last_marked_idx
            );
            // Only delete if we can remove everything up to and including the last marked clipboard
            if total_to_delete > last_marked_idx {
                // Delete oldest clipboards including the marked ones
                for clippy in all_clipboards.iter().take(total_to_delete) {
                    printlog!(
                        "deleting clipboard: {} from {} (marked: {})",
                        clippy.id,
                        uuid_to_datetime(&clippy.id),
                        clippy.deleted_at.is_some()
                    );
                    self.delete_clipboard(clippy).await;
                }
            }
        } else {
            // No marked clipboards - normal cleanup
            let to_delete = all_clipboards.len() - sync_limit;
            for clippy in all_clipboards.iter().take(to_delete) {
                printlog!(
                    "deleting clipboard: {} from {}",
                    clippy.id,
                    uuid_to_datetime(&clippy.id),
                );
                self.delete_clipboard(clippy).await;
            }
        }

        Ok(())
    }

    async fn upload_clipboard(
        &self,
        clipboard: &FullClipboardDto,
    ) -> Result<Clippy, Box<dyn std::error::Error>> {
        let file_name = create_clipboard_filename(
            &clipboard.clipboard.id,
            &clipboard.clipboard.star,
            &clipboard.clipboard.encrypted,
            &clipboard.clipboard.created_at,
            None,
        );

        printlog!(
            "uploading clipboard: {} from {} star: {} encrypted: {}",
            clipboard.clipboard.id,
            uuid_to_datetime(&clipboard.clipboard.id),
            clipboard.clipboard.star,
            clipboard.clipboard.encrypted
        );

        let file = File {
            name: Some(file_name),
            mime_type: Some("application/json".into()),
            created_time: Some(Utc.from_utc_datetime(&clipboard.clipboard.created_at)),
            parents: Some(vec!["appDataFolder".into()]),
            ..Default::default()
        };

        let (_, file) = self
            .0
            .hub
            .files()
            .create(file)
            .add_scope(Scope::Appdata.as_ref())
            .upload(
                Cursor::new(serde_json::to_string(&clipboard)?),
                "application/json".parse()?,
            )
            .await?;

        Ok(parse_clipboard_info(
            file.name.as_ref().expect("No name"),
            file.id.as_ref().expect("No id"),
        )
        .expect("Failed to parse clipboard info"))
    }

    async fn update_clipboard(
        &self,
        local_clipboard: &FullClipboardDto,
        remote_clipboard: &Clippy,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create new filename based on clipboard properties
        let new_name = create_clipboard_filename(
            &remote_clipboard.id,
            &local_clipboard.clipboard.star, // Use local star status
            &local_clipboard.clipboard.encrypted,
            &remote_clipboard.created_at,
            None,
        );

        printlog!(
            "updating clipboard: {} from {} star: {} encrypted: {}",
            remote_clipboard.id,
            uuid_to_datetime(&remote_clipboard.id),
            local_clipboard.clipboard.star,
            local_clipboard.clipboard.encrypted
        );

        // Create metadata update
        let file = google_drive3::api::File {
            name: Some(new_name),
            mime_type: Some("application/json".into()),
            ..Default::default()
        };

        // Update the file metadata and content
        self.0
            .hub
            .files()
            .update(file, &remote_clipboard.provider_id)
            .add_scope(Scope::Appdata.as_ref())
            .upload(
                Cursor::new(serde_json::to_string(&local_clipboard)?),
                "application/json".parse()?,
            )
            .await?;

        Ok(())
    }

    async fn star_clipboard(&self, clippy: &FullClipboardDto) {
        let clipboards = self
            .fetch_all_clipboards()
            .await
            .expect("Failed to fetch clipboards");

        let remote_clipboards = clipboards
            .iter()
            .find(|clip| clip.id == clippy.clipboard.id);

        if let Some(remote_clipboard) = remote_clipboards {
            let new_name = create_clipboard_filename(
                &remote_clipboard.id,
                &remote_clipboard.star,
                &remote_clipboard.encrypted,
                &remote_clipboard.created_at,
                None,
            );

            let file = google_drive3::api::File {
                name: Some(new_name),
                ..Default::default()
            };

            self.0
                .hub
                .files()
                .update(file, &remote_clipboard.provider_id)
                .add_scope(Scope::Appdata.as_ref())
                .doit_without_upload()
                .await
                .expect("Failed to rename file");
        } else {
            self.upload_clipboard(clippy)
                .await
                .expect("Failed to upload clipboard");
        }
    }

    async fn upsert_settings(
        &self,
        settings: &HashMap<String, Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = File {
            name: Some(format!("{}.json", BACKUP_SETTINGS_PREFIX)),
            mime_type: Some("application/json".into()),
            parents: Some(vec!["appDataFolder".into()]),
            ..Default::default()
        };

        // Upload the settings (will overwrite if exists)
        let (_, _) = self
            .0
            .hub
            .files()
            .create(file)
            .add_scope(Scope::Appdata.as_ref())
            .upload(
                Cursor::new(serde_json::to_string(settings)?),
                "application/json".parse()?,
            )
            .await?;

        printlog!("(remote) uploaded settings");

        Ok(())
    }

    async fn get_settings(
        &self,
    ) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
        let file = match self.find_settings_file().await? {
            Some(f) => f,
            None => return Ok(HashMap::new()),
        };

        let (mut response, _) = self
            .0
            .hub
            .files()
            .get(&file.id.expect("No id"))
            .param("alt", "media")
            .acknowledge_abuse(true)
            .add_scope(Scope::Appdata.as_ref())
            .doit()
            .await?;

        let content = String::from_utf8(response.body_mut().collect().await?.to_bytes().to_vec())?;

        let settings: HashMap<String, serde_json::Value> = serde_json::from_str(&content)?;

        Ok(settings)
    }

    async fn is_authenticated(&self) -> bool {
        match self.0.hub.auth.get_token(&[Scope::Appdata.as_ref()]).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
