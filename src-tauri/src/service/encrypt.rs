use super::clipboard::load_clipboards_with_relations;
use super::decrypt::init_password_lock;
use super::sync::get_sync_provider;
use crate::prelude::*;
use crate::service::clipboard::upsert_clipboard_dto;
use crate::service::settings::get_global_settings;
use crate::tao::connection::db;
use crate::tao::global::get_app;
use base64::{engine::general_purpose::STANDARD, Engine};
use common::types::crypto::{EncryptionError, ENCRYPTION_KEY};
use common::types::enums::ListenEvent;
use common::types::orm_query::FullClipboardDto;
use common::types::types::{CommandError, Progress};
use entity::clipboard;
use ring::rand::SecureRandom;
use ring::{aead, rand};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tauri::{Emitter, EventTarget};

pub async fn encrypt_all_clipboards() -> Result<(), CommandError> {
    let settings = get_global_settings();
    let db = db().await?;

    let clipboards = load_clipboards_with_relations(
        clipboard::Entity::find()
            .filter(clipboard::Column::Encrypted.eq(false))
            .all(&db)
            .await?,
    )
    .await;

    let (provider, remote_clipboards) = if settings.sync {
        let provider = get_sync_provider().await;
        (
            Some(provider.clone()),
            provider
                .fetch_all_clipboards()
                .await
                .expect("Failed to fetch remote clipboards"),
        )
    } else {
        (None, Vec::new())
    };

    let total_clipboards = clipboards.len() as u64;
    for (index, clipboard) in clipboards.into_iter().enumerate() {
        let encrypted_clipboard = encrypt_clipboard(clipboard);

        // Update clipboard in database
        upsert_clipboard_dto(encrypted_clipboard.clone()).await?;

        if let Some(provider) = &provider {
            if let Some(remote_clipboards) = &remote_clipboards
                .iter()
                .find(|c| c.id == encrypted_clipboard.clipboard.id)
            {
                provider
                    .update_clipboard(&encrypted_clipboard, &remote_clipboards)
                    .await
                    .expect("Failed to upsert clipboard");
            }
        }

        let progress = Progress {
            total: total_clipboards as u64,
            current: (index + 1) as u64,
        };

        printlog!("Emitting progress event {:?}", progress);

        get_app()
            .emit_to(
                EventTarget::any(),
                ListenEvent::Progress.to_string().as_str(),
                progress,
            )
            .expect("Failed to emit download progress event");
    }

    Ok(())
}

pub fn encrypt_clipboard(mut clipboard: FullClipboardDto) -> FullClipboardDto {
    if let Some(text) = &mut clipboard.text {
        if !STANDARD.decode(text.data.clone()).is_ok() {
            text.data = STANDARD
                .encode(encrypt_data(text.data.as_bytes()).expect("Text encryption failed"));
        }
    }

    if let Some(html) = &mut clipboard.html {
        if !STANDARD.decode(html.data.clone()).is_ok() {
            html.data = STANDARD
                .encode(encrypt_data(html.data.as_bytes()).expect("HTML encryption failed"));
        }
    }

    if let Some(rtf) = &mut clipboard.rtf {
        if !STANDARD.decode(rtf.data.clone()).is_ok() {
            rtf.data =
                STANDARD.encode(encrypt_data(rtf.data.as_bytes()).expect("RTF encryption failed"));
        }
    }

    if let Some(image) = &mut clipboard.image {
        image.data = encrypt_data(image.data.as_slice()).expect("Image encryption failed");
        if let Ok(thumbnail_bytes) = STANDARD.decode(image.thumbnail.clone()) {
            let encrypted_thumbnail =
                encrypt_data(&thumbnail_bytes).expect("Thumbnail encryption failed");
            image.thumbnail = STANDARD.encode(&encrypted_thumbnail);
        }
    }

    if !clipboard.files.is_empty() {
        for file in &mut clipboard.files {
            if !STANDARD.decode(file.name.clone()).is_ok() {
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

pub fn setup_encryption() {
    let settings = get_global_settings();
    if !is_key_set() && settings.encryption {
        init_password_lock();
    } else {
        printlog!("Encryption key is set or encryption is disabled");
    }
}

/// Sets the encryption key derived from a password
pub fn set_encryption_key(password: &str) -> Result<(), EncryptionError> {
    let mut hasher = ring::digest::Context::new(&ring::digest::SHA256);
    hasher.update(password.as_bytes());
    let key = hasher.finish();
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(key.as_ref());

    *ENCRYPTION_KEY
        .lock()
        .map_err(|_| EncryptionError::KeyLockFailed)? = Some(key_bytes);

    Ok(())
}

/// Checks if encryption key is set
pub fn is_key_set() -> bool {
    ENCRYPTION_KEY.lock().map(|k| k.is_some()).unwrap_or(false)
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

    // Combine nonce and encrypted data
    Ok([nonce_bytes.to_vec(), in_out].concat())
}

/// Checks if data appears to be encrypted based on its structure
pub fn looks_like_encrypted_data(data: &[u8]) -> bool {
    // Check minimum size (nonce + tag)
    if data.len() < 12 + 16 {
        return false;
    }

    // Check AES block alignment
    if (data.len() - 12) % 16 != 0 {
        return false;
    }

    true
}
