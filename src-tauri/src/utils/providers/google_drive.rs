use super::parse_clipboard_info;
use crate::{
    service::settings::{get_settings_db, update_settings_synchronize_db},
    utils::providers::create_clipboard_filename,
};
use chrono::{NaiveDateTime, TimeZone, Utc};
use common::{
    constants::{BACKUP_FILE_PREFIX, BACKUP_SETTINGS_PREFIX, TOKEN_NAME},
    printlog,
    types::{
        orm_query::FullClipboardDto,
        sync::{ClippyInfo, GoogleDriveProvider, SyncProvider},
        types::CommandError,
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
use tao::{config::get_data_path, global::get_app};
use tauri::Manager;
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
        let secret = yup_oauth2::ApplicationSecret {
            client_id: std::env::var("GOOGLE_CLIENT_ID")?,
            client_secret: std::env::var("GOOGLE_CLIENT_SECRET")?,
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

        printlog!("(remote) found {} clipboards", all_files.len());

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
    async fn fetch_all_clipboards(&self) -> Result<Vec<ClippyInfo>, Box<dyn std::error::Error>> {
        let filelist = self.fetch_all_clipboard_files().await?;
        let mut clipboards = Vec::new();

        for file in filelist {
            if let Some(remote) =
                parse_clipboard_info(&file.name.as_ref().expect("No name"), file.id)
            {
                clipboards.push(remote);
            }
        }

        Ok(clipboards)
    }

    async fn fetch_new_clipboards(
        &self,
        local_clipboards: &HashMap<Uuid, NaiveDateTime>,
        remote_clipboards: &Vec<ClippyInfo>,
    ) -> Result<Vec<FullClipboardDto>, Box<dyn std::error::Error>> {
        let mut clipboards = Vec::new();

        for file in remote_clipboards {
            if let Some(existing_timestamp) = local_clipboards.get(&file.id) {
                if existing_timestamp >= &file.timestamp {
                    continue;
                }
            }

            printlog!(
                "downloading clipboard: {} from {} starred: {}",
                file.id,
                file.timestamp,
                file.starred
            );

            if let Some(id) = &file.provider_id {
                clipboards.push(self.download_by_id(id).await?);
            }
        }

        Ok(clipboards)
    }

    async fn upload_new_clipboards(
        &self,
        new_local_clipboards: &[FullClipboardDto],
        remote_clipboards: &mut Vec<ClippyInfo>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let remote_map: HashMap<Uuid, NaiveDateTime> = remote_clipboards
            .iter()
            .map(|clip| (clip.id, clip.timestamp))
            .collect();

        let remote_count = remote_clipboards.len();

        for clipboard in new_local_clipboards {
            if let Some(remote_clipboard) = remote_map.get(&clipboard.clipboard.id) {
                if remote_clipboard >= &clipboard.clipboard.created_date {
                    continue;
                }
            }

            let remote_clipboard = self.upload_clipboard(clipboard).await?;

            remote_clipboards.push(remote_clipboard);
        }

        Ok(remote_clipboards.len() > remote_count)
    }

    async fn delete_by_id(&self, id: &String) {
        self.0
            .hub
            .files()
            .delete(&id)
            .add_scope(Scope::Appdata.as_ref())
            .doit()
            .await
            .expect("Failed to delete clipboard");

        printlog!("deleted clipboard: {}", id);
    }

    async fn delete_by_uuid(&self, uuid: &Uuid) {
        let files = self
            .fetch_all_clipboard_files()
            .await
            .expect("Failed to fetch clipboards");
        for file in files {
            if let Some(remote) = parse_clipboard_info(&file.name.unwrap(), file.id.clone()) {
                if remote.id == *uuid {
                    self.delete_by_id(&file.id.as_ref().expect("No id")).await;
                    break;
                }
            }
        }
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
        remote_clipboards: &Vec<ClippyInfo>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let settings = get_settings_db().await?;

        if remote_clipboards.len() <= settings.sync_limit as usize {
            return Ok(());
        }

        let mut clipboards = remote_clipboards
            .iter()
            .filter(|clip| !clip.starred)
            .map(|file| (file.provider_id.as_ref().expect("No provider id"), file))
            .collect::<Vec<_>>();

        clipboards.sort_by_key(|(_, info)| info.timestamp);

        let files_to_delete = clipboards
            .len()
            .saturating_sub(settings.sync_limit as usize);

        for (id, info) in clipboards.into_iter().take(files_to_delete) {
            printlog!(
                "deleting clipboard: {} from {} starred: {}",
                info.id,
                info.timestamp,
                info.starred
            );

            self.delete_by_id(id).await;
        }

        Ok(())
    }

    async fn upload_clipboard(
        &self,
        clipboard: &FullClipboardDto,
    ) -> Result<ClippyInfo, Box<dyn std::error::Error>> {
        let file_name = create_clipboard_filename(
            &clipboard.clipboard.id,
            &clipboard.clipboard.created_date,
            &clipboard.clipboard.star,
        );

        printlog!(
            "uploading clipboard: {} from {} starred: {}",
            clipboard.clipboard.id,
            clipboard.clipboard.created_date,
            clipboard.clipboard.star
        );

        let file = File {
            name: Some(file_name),
            mime_type: Some("application/json".into()),
            created_time: Some(Utc.from_utc_datetime(&clipboard.clipboard.created_date)),
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

        Ok(
            parse_clipboard_info(file.name.as_ref().expect("No name"), file.id)
                .expect("Failed to parse clipboard info"),
        )
    }

    async fn upsert_settings(
        &self,
        settings: &HashMap<String, Value>,
    ) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
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

        Ok(settings.clone())
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
            .get(&file.id.unwrap())
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
