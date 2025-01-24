use super::clipboard::load_clipboards_with_relations;
use super::sync::get_sync_provider;
use crate::prelude::*;
use crate::service::settings::get_global_settings;
use base64::{engine::general_purpose::STANDARD, Engine};
use common::types::crypto::{EncryptionError, ENCRYPTION_KEY};
use common::types::enums::ListenEvent;
use common::types::orm_query::FullClipboardDto;
use common::types::types::{CommandError, Progress};
use entity::{
    clipboard, clipboard_file, clipboard_html, clipboard_image, clipboard_rtf, clipboard_text,
};
use ring::rand::SecureRandom;
use ring::{aead, rand};
use sea_orm::EntityTrait;
use tao::connection::db;
use tao::global::get_app;
use tauri::{Emitter, EventTarget};

pub async fn encrypt_all_clipboards() -> Result<(), CommandError> {
    let settings = get_global_settings();
    let db = db().await?;

    let clipboards =
        load_clipboards_with_relations(clipboard::Entity::find().all(&db).await?).await;

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
        let encrypted_clipboard = encrypt_clipboard(clipboard).await?;

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

        let progress: Progress = Progress {
            total: total_clipboards as u64,
            current: (index + 1) as u64,
        };

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

pub async fn encrypt_clipboard(
    mut clipboard: FullClipboardDto,
) -> Result<FullClipboardDto, CommandError> {
    let db = db().await?;

    if let Some(text) = &mut clipboard.text {
        if !looks_like_encrypted_data(text.data.as_bytes()) {
            text.data = String::from_utf8(encrypt_data(text.data.as_bytes())?).unwrap_or_default();

            let clipboard_text: clipboard_text::ActiveModel = text.clone().into();
            clipboard_text::Entity::update(clipboard_text)
                .exec(&db)
                .await?;
        }
    }

    if let Some(html) = &mut clipboard.html {
        if !looks_like_encrypted_data(html.data.as_bytes()) {
            html.data = String::from_utf8(encrypt_data(html.data.as_bytes())?).unwrap_or_default();

            let clipboard_html: clipboard_html::ActiveModel = html.clone().into();
            clipboard_html::Entity::update(clipboard_html)
                .exec(&db)
                .await?;
        }
    }

    if let Some(rtf) = &mut clipboard.rtf {
        if !looks_like_encrypted_data(rtf.data.as_bytes()) {
            rtf.data = String::from_utf8(encrypt_data(rtf.data.as_bytes())?).unwrap_or_default();

            let clipboard_rtf: clipboard_rtf::ActiveModel = rtf.clone().into();
            clipboard_rtf::Entity::update(clipboard_rtf)
                .exec(&db)
                .await?;
        }
    }

    if let Some(image) = &mut clipboard.image {
        if !looks_like_encrypted_data(image.data.as_slice()) {
            image.data = encrypt_data(image.data.as_slice())?;
            if let Ok(thumbnail_bytes) = STANDARD.decode(image.thumbnail.clone()) {
                let encrypted_thumbnail = encrypt_data(&thumbnail_bytes)?;
                image.thumbnail = STANDARD.encode(&encrypted_thumbnail);
            }
            let image: clipboard_image::ActiveModel = image.clone().into();
            clipboard_image::Entity::update(image).exec(&db).await?;
        }
    }

    if !clipboard.files.is_empty() {
        for file in &mut clipboard.files {
            if !looks_like_encrypted_data(file.data.as_slice()) {
                file.data = encrypt_data(file.data.as_slice())?;

                if let Some(name) = &file.name {
                    file.name =
                        Some(String::from_utf8(encrypt_data(name.as_bytes())?).unwrap_or_default());
                }

                if let Some(extension) = &file.extension {
                    file.extension = Some(
                        String::from_utf8(encrypt_data(extension.as_bytes())?).unwrap_or_default(),
                    );
                }

                if let Some(mime_type) = &file.mime_type {
                    file.mime_type = Some(
                        String::from_utf8(encrypt_data(mime_type.as_bytes())?).unwrap_or_default(),
                    );
                }

                let file: clipboard_file::ActiveModel = file.clone().into();
                clipboard_file::Entity::update(file).exec(&db).await?;
            }
        }
    }

    clipboard.clipboard.encrypted = true;
    let entity: clipboard::ActiveModel = clipboard.clipboard.clone().into();
    clipboard::Entity::update(entity).exec(&db).await?;

    Ok(clipboard)
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
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))? = Some(key_bytes);
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
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?
        .ok_or(EncryptionError::NoKey)?;

    // Create unbound key from key bytes
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
        .map_err(|_| EncryptionError::EncryptionFailed("Failed to create key".to_string()))?;
    let key = aead::LessSafeKey::new(unbound_key);

    // Generate random nonce
    let rng = rand::SystemRandom::new();
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes)
        .map_err(|_| EncryptionError::EncryptionFailed("Failed to generate nonce".to_string()))?;
    let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);

    // Encrypt data
    let mut in_out = data.to_vec();
    key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)
        .map_err(|_| EncryptionError::EncryptionFailed("Encryption failed".to_string()))?;

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
