use super::clipboard::{init_clipboards, load_clipboards_with_relations};
use super::sync::{get_sync_manager, get_sync_provider};
use crate::prelude::*;
use crate::service::clipboard::upsert_clipboard_dto;
use crate::service::settings::get_global_settings;
use crate::tao::connection::db;
use crate::tao::global::get_app;
use base64::{engine::general_purpose::STANDARD, Engine};
use common::constants::ENCRYPTION_MAGIC_STRING;
use common::types::cipher::{EncryptionError, ENCRYPTION_KEY};
use common::types::enums::ListenEvent;
use common::types::orm_query::FullClipboardDto;
use common::types::types::{CommandError, Progress};
use entity::clipboard;
use ring::rand::SecureRandom;
use ring::{aead, rand};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::thread::sleep;
use tauri::{Emitter, EventTarget};

pub async fn encrypt_all_clipboards(full: bool) -> Result<(), CommandError> {
    if full {
        encrypt_all_clipboards_internal().await
    } else {
        // Spawn a new task to run in a separate thread if `full` is false
        tauri::async_runtime::spawn(async {
            if let Err(e) = encrypt_all_clipboards_internal().await {
                eprintln!("Error encrypting clipboards: {:?}", e);
            }
        });

        Ok(())
    }
}

async fn encrypt_all_clipboards_internal() -> Result<(), CommandError> {
    let settings = get_global_settings();
    let db = db().await?;

    // Get all local unencrypted clipboards
    let mut clipboards = load_clipboards_with_relations(
        clipboard::Entity::find()
            .filter(clipboard::Column::Encrypted.eq(false))
            .all(&db)
            .await?,
    )
    .await;

    // Get remote clipboards if sync enabled
    let (provider, remote_clipboards) = if settings.sync {
        // Stop the sync manager before making changes
        get_sync_manager().lock().await.stop().await;

        let provider = get_sync_provider().await;
        let remote_clipboards = provider
            .fetch_all_clipboards()
            .await
            .expect("Failed to fetch remote clipboards");

        // Download all remote clipboards with progress logging
        let download_total = remote_clipboards.len();
        for (index, remote) in remote_clipboards.iter().enumerate() {
            if remote.encrypted {
                continue;
            }

            if remote.deleted_at.is_some() {
                continue;
            }

            if clipboards.iter().any(|c| c.clipboard.id == remote.id) {
                continue;
            }

            if let Ok(clipboard) = provider.download_by_id(&remote.provider_id).await {
                clipboards.push(clipboard);

                get_app().emit_to(
                    EventTarget::any(),
                    ListenEvent::Progress.to_string().as_str(),
                    Progress {
                        label: "SETTINGS.ENCRYPT.DOWNLOADING_REMOTE_CLIPBOARDS".to_string(),
                        total: download_total,
                        current: index + 1,
                    },
                )?;
            }
        }
        (Some(provider), remote_clipboards)
    } else {
        (None, Vec::new())
    };

    let total = clipboards.len();

    for (index, clipboard) in clipboards.into_iter().enumerate() {
        let encrypted = encrypt_clipboard(clipboard);
        upsert_clipboard_dto(encrypted.clone()).await?;

        get_app().emit_to(
            EventTarget::any(),
            ListenEvent::Progress.to_string().as_str(),
            Progress {
                label: "SETTINGS.ENCRYPT.ENCRYPTION_PROGRESS_LOCAL".to_string(),
                total,
                current: index + 1,
            },
        )?;

        if let Some(provider) = &provider {
            if let Some(remote) = remote_clipboards
                .iter()
                .find(|r| r.id == encrypted.clipboard.id)
            {
                provider.update_clipboard(&encrypted, remote).await.ok();
            }
        }
    }

    if settings.sync && provider.is_some() {
        // race condition with settings sync
        tauri::async_runtime::spawn(async {
            sleep(std::time::Duration::from_secs(5));
            get_sync_manager().lock().await.start().await;
        });
    }

    init_clipboards();

    Ok(())
}

pub fn encrypt_clipboard(mut clipboard: FullClipboardDto) -> FullClipboardDto {
    if let Some(text) = &mut clipboard.text {
        if !looks_like_encrypted_data(text.data.as_bytes()) {
            text.data = STANDARD
                .encode(encrypt_data(text.data.as_bytes()).expect("Text encryption failed"));
        }
    }

    if let Some(html) = &mut clipboard.html {
        if !looks_like_encrypted_data(html.data.as_bytes()) {
            html.data = STANDARD
                .encode(encrypt_data(html.data.as_bytes()).expect("HTML encryption failed"));
        }
    }

    if let Some(rtf) = &mut clipboard.rtf {
        if !looks_like_encrypted_data(rtf.data.as_bytes()) {
            rtf.data =
                STANDARD.encode(encrypt_data(rtf.data.as_bytes()).expect("RTF encryption failed"));
        }
    }

    if let Some(image) = &mut clipboard.image {
        if !looks_like_encrypted_data(&image.data) {
            image.data = encrypt_data(image.data.as_slice()).expect("Image encryption failed");
        }

        if let Ok(thumbnail_bytes) = STANDARD.decode(&image.thumbnail) {
            if !looks_like_encrypted_data(&thumbnail_bytes) {
                let encrypted_thumbnail =
                    encrypt_data(&thumbnail_bytes).expect("Thumbnail encryption failed");
                image.thumbnail = STANDARD.encode(&encrypted_thumbnail);
            }
        }
    }

    if !clipboard.files.is_empty() {
        for file in &mut clipboard.files {
            if !looks_like_encrypted_data(file.name.as_bytes()) {
                file.data =
                    encrypt_data(file.data.as_slice()).expect("File data encryption failed");

                file.name = STANDARD.encode(
                    encrypt_data(file.name.as_bytes()).expect("Filename encryption failed"),
                );

                if let Some(extension) = &file.extension {
                    file.extension = Some(
                        STANDARD.encode(
                            encrypt_data(extension.as_bytes())
                                .expect("File extension encryption failed"),
                        ),
                    );
                }

                if let Some(mime_type) = &file.mime_type {
                    file.mime_type = Some(STANDARD.encode(
                        encrypt_data(mime_type.as_bytes()).expect("MIME type encryption failed"),
                    ));
                }
            }
        }
    }

    clipboard.clipboard.encrypted = true;
    clipboard
}

/// Encrypts data using AES-256-GCM
pub fn encrypt_data(data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    let key_bytes = ENCRYPTION_KEY
        .lock()
        .map_err(|_| EncryptionError::KeyLockFailed)?
        .ok_or(EncryptionError::NoKey)?;

    // Create unbound key from key bytes
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
        .map_err(|_| EncryptionError::EncryptionFailed)?;
    let key = aead::LessSafeKey::new(unbound_key);

    // Generate random nonce
    let rng = rand::SystemRandom::new();
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes)
        .map_err(|_| EncryptionError::EncryptionFailed)?;
    let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);

    // Encrypt data
    let mut in_out = data.to_vec();
    key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)
        .map_err(|_| EncryptionError::EncryptionFailed)?;

    // Combine magic string bytes, nonce, and encrypted data
    Ok([
        ENCRYPTION_MAGIC_STRING.as_bytes().to_vec(),
        nonce_bytes.to_vec(),
        in_out,
    ]
    .concat())
}

/// Checks if data appears to be encrypted based on its structure
pub fn looks_like_encrypted_data(data: &[u8]) -> bool {
    let magic_bytes = ENCRYPTION_MAGIC_STRING.as_bytes();

    // Check minimum length and magic bytes
    data.len() >= magic_bytes.len() && data[..magic_bytes.len()] == *magic_bytes
}
