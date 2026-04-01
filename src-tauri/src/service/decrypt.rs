use super::{
    cipher::{clear_encryption_key, is_encryption_key_set, verify_encryption_password},
    clipboard::load_clipboards_with_relations,
    settings::{get_global_settings, update_settings_db},
    sync::{get_sync_manager, get_sync_provider},
};
use crate::{
    prelude::*,
    service::clipboard::upsert_clipboard_dto,
    tao::{connection::db, global::get_app},
};
use base64::{engine::general_purpose::STANDARD, Engine};
use common::{constants::ENCRYPTION_MAGIC_STRING, types::{
    cipher::{EncryptionError, ENCRYPTION_KEY},
    enums::ListenEvent,
    orm_query::FullClipboardDto,
    types::{CommandError, Progress},
}};
use entity::clipboard;
use ring::aead;
use sea_orm::prelude::Uuid;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tauri::{Emitter, EventTarget};

pub async fn decrypt_all_clipboards() -> Result<(), CommandError> {
    let settings = get_global_settings();
    let db = db();

    // Get all local encrypted clipboards
    let mut clipboards = load_clipboards_with_relations(
        clipboard::Entity::find()
            .filter(clipboard::Column::Encrypted.eq(true))
            .all(db)
            .await?,
    )
    .await;

    // Get remote clipboards if sync enabled
    let (provider, remote_clipboards) = if settings.sync {
        let provider = get_sync_provider().await;
        let remote_clipboards = provider
            .fetch_all_clipboards()
            .await
            .expect("Failed to fetch remote clipboards");

        // Download all remote clipboards with progress logging
        let download_total = remote_clipboards.len();
        for (index, remote) in remote_clipboards.iter().enumerate() {
            get_app().emit_to(
                EventTarget::any(),
                ListenEvent::Progress.to_string().as_str(),
                Progress {
                    label: "SETTINGS.ENCRYPT.DOWNLOADING_REMOTE_CLIPBOARDS".to_string(),
                    total: download_total,
                    current: index + 1,
                },
            )?;

            if !remote.encrypted {
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
            }
        }
        (Some(provider), remote_clipboards)
    } else {
        (None, Vec::new())
    };

    let total = clipboards.len();
    for (index, clipboard) in clipboards.into_iter().enumerate() {
        get_app().emit_to(
            EventTarget::any(),
            ListenEvent::Progress.to_string().as_str(),
            Progress {
                label: "SETTINGS.ENCRYPT.DECRYPTION_PROGRESS".to_string(),
                total,
                current: index + 1,
            },
        )?;
        match decrypt_clipboard(clipboard.clone()) {
            Ok(decrypted) => {
                upsert_clipboard_dto(decrypted.clone()).await?;

                if let Some(provider) = &provider {
                    if let Some(remote) = remote_clipboards
                        .iter()
                        .find(|r| r.id == decrypted.clipboard.id)
                    {
                        provider.update_clipboard(&decrypted, remote).await.ok();
                    }
                }
            }
            Err(e) => {
                printlog!(
                    "Failed to decrypt clipboard {}: {:?}",
                    clipboard.clipboard.id,
                    e
                );

                // Delete locally
                clipboard::Entity::delete_by_id(clipboard.clipboard.id)
                    .exec(db)
                    .await?;

                // Mark for deletion remotely if sync is enabled
                if let Some(provider) = &provider {
                    if let Some(remote) = remote_clipboards
                        .iter()
                        .find(|r| r.id == clipboard.clipboard.id)
                    {
                        provider.mark_for_deletion(remote).await;
                    }
                }
            }
        }
    }

    Ok(())
}

/// Decrypts a base64-encoded encrypted string field
fn decrypt_string_field(
    data: &str,
    clipboard_id: Uuid,
    field_name: &str,
) -> Result<String, EncryptionError> {
    let decoded = STANDARD.decode(data).map_err(|e| {
        printlog!(
            "Failed to base64 decode {} for clipboard {}: {}",
            field_name,
            clipboard_id,
            e
        );
        EncryptionError::DecryptionFailed
    })?;
    let decrypted = decrypt_data(&decoded).map_err(|e| {
        printlog!(
            "Failed to decrypt {} for clipboard {}: {:?}",
            field_name,
            clipboard_id,
            e
        );
        e
    })?;
    String::from_utf8(decrypted).map_err(|e| {
        printlog!(
            "Failed to convert decrypted {} to UTF-8 for clipboard {}: {}",
            field_name,
            clipboard_id,
            e
        );
        EncryptionError::DecryptionFailed
    })
}

/// Decrypts raw binary encrypted data
fn decrypt_binary_field(
    data: &[u8],
    clipboard_id: Uuid,
    field_name: &str,
) -> Result<Vec<u8>, EncryptionError> {
    decrypt_data(data).map_err(|e| {
        printlog!(
            "Failed to decrypt {} for clipboard {}: {:?}",
            field_name,
            clipboard_id,
            e
        );
        e
    })
}

pub fn decrypt_clipboard(
    mut clipboard: FullClipboardDto,
) -> Result<FullClipboardDto, EncryptionError> {
    if !clipboard.clipboard.encrypted {
        return Err(EncryptionError::NotEncrypted);
    }
    let id = clipboard.clipboard.id;

    if let Some(text) = &mut clipboard.text {
        text.data = decrypt_string_field(&text.data, id, "text")?;
    }

    if let Some(html) = &mut clipboard.html {
        html.data = decrypt_string_field(&html.data, id, "html")?;
    }

    if let Some(rtf) = &mut clipboard.rtf {
        rtf.data = decrypt_string_field(&rtf.data, id, "rtf")?;
    }

    if let Some(image) = &mut clipboard.image {
        image.data = decrypt_binary_field(&image.data, id, "image")?;

        let thumb_decoded = STANDARD.decode(&image.thumbnail).map_err(|e| {
            printlog!(
                "Failed to base64 decode thumbnail for clipboard {}: {}",
                id,
                e
            );
            EncryptionError::DecryptionFailed
        })?;
        image.thumbnail =
            STANDARD.encode(decrypt_binary_field(&thumb_decoded, id, "thumbnail")?);

        if let Some(ocr_text) = &image.ocr_text {
            // OCR text might not be encrypted (e.g. added after encryption was enabled)
            if let Ok(decoded) = STANDARD.decode(ocr_text) {
                if let Ok(decrypted) = decrypt_data(&decoded) {
                    if let Ok(str_data) = String::from_utf8(decrypted) {
                        image.ocr_text = Some(str_data);
                    }
                }
            }
        }
    }

    for (i, file) in clipboard.files.iter_mut().enumerate() {
        file.name = decrypt_string_field(&file.name, id, &format!("file[{}].name", i))?;
        file.data = decrypt_binary_field(&file.data, id, &format!("file[{}].data", i))?;

        if let Some(ext) = &file.extension {
            file.extension =
                Some(decrypt_string_field(ext, id, &format!("file[{}].ext", i))?);
        }
        if let Some(mime) = &file.mime_type {
            file.mime_type =
                Some(decrypt_string_field(mime, id, &format!("file[{}].mime", i))?);
        }
    }

    clipboard.clipboard.encrypted = false;
    Ok(clipboard)
}

/// Decrypts data using AES-256-GCM
pub fn decrypt_data(encrypted_data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    let magic_bytes = ENCRYPTION_MAGIC_STRING.as_bytes();

    // Validate input has minimum required length
    let min_length = magic_bytes.len() + 12 + 16; // magic bytes + nonce + minimum tag size
    if encrypted_data.len() < min_length {
        return Err(EncryptionError::NotEncrypted);
    }

    // Verify magic bytes
    if &encrypted_data[..magic_bytes.len()] != magic_bytes {
        return Err(EncryptionError::NotEncrypted);
    }

    let guard = ENCRYPTION_KEY
        .lock()
        .map_err(|_| EncryptionError::KeyLockFailed)?;
    let key_data = guard.as_ref().ok_or(EncryptionError::NoKey)?;

    // Create unbound key from key bytes
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_data.0)
        .map_err(|_| EncryptionError::DecryptionFailed)?;
    let key = aead::LessSafeKey::new(unbound_key);

    // Extract nonce (after magic bytes)
    let magic_bytes_len = magic_bytes.len();
    let nonce = aead::Nonce::assume_unique_for_key(
        encrypted_data[magic_bytes_len..magic_bytes_len + 12]
            .try_into()
            .map_err(|_| EncryptionError::DecryptionFailed)?,
    );

    // Get encrypted data (after magic bytes and nonce)
    let mut in_out = encrypted_data[magic_bytes_len + 12..].to_vec();

    // Decrypt in place
    match key.open_in_place(nonce, aead::Aad::empty(), &mut in_out) {
        Ok(decrypted) => Ok(decrypted.to_vec()),
        Err(_) => Err(EncryptionError::InvalidKey),
    }
}

pub async fn remove_encryption(password: String) -> Result<(), CommandError> {
    if !is_encryption_key_set() {
        return Err(CommandError::new("MAIN.ERROR.NO_ENCRYPTION_KEY_SET"));
    }

    let is_password_valid =
        verify_encryption_password(password).map_err(|e| CommandError::new(&e.to_string()))?;

    if !is_password_valid {
        return Err(CommandError::new("MAIN.ERROR.INCORRECT_PASSWORD"));
    }

    let was_syncing = get_global_settings().sync;

    // Stop the sync manager before making changes
    get_sync_manager().lock().await.stop().await;
    decrypt_all_clipboards().await?;

    let mut settings = get_global_settings();
    settings.encryption = false;
    update_settings_db(settings).await?;

    // Restart sync manager after settings are persisted
    if was_syncing {
        get_sync_manager().lock().await.start().await;
    }

    clear_encryption_key();

    Ok(())
}
