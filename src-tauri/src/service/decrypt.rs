use super::{
    clipboard::load_clipboards_with_relations,
    encrypt::{is_key_set, looks_like_encrypted_data},
    settings::{get_global_settings, update_settings_db},
    sync::get_sync_provider,
};
use crate::{
    prelude::*,
    service::clipboard::upsert_clipboard_dto,
    tao::{connection::db, global::get_app},
};
use base64::{engine::general_purpose::STANDARD, Engine};
use common::{
    printlog,
    types::{
        cipher::{EncryptionError, ENCRYPTION_KEY},
        enums::ListenEvent,
        orm_query::FullClipboardDto,
        types::{CommandError, Progress},
    },
};
use entity::clipboard;
use ring::aead;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tauri::{Emitter, EventTarget};

pub async fn decrypt_all_clipboards() -> Result<(), CommandError> {
    let settings = get_global_settings();
    let db = db().await?;

    let clipboards = load_clipboards_with_relations(
        clipboard::Entity::find()
            .filter(clipboard::Column::Encrypted.eq(true))
            .all(&db)
            .await?,
    )
    .await;

    let (remote_clipboards, provider) = if settings.sync {
        let provider = get_sync_provider().await;
        (
            provider
                .fetch_all_clipboards()
                .await
                .expect("Failed to fetch remote clipboards"),
            Some(provider),
        )
    } else {
        (Vec::new(), None)
    };

    let total_clipboards = clipboards.len() as u64;
    for (index, clipboard) in clipboards.into_iter().enumerate() {
        let decrypted_clipboard =
            decrypt_clipboard(clipboard).expect("Failed to decrypt clipboard");

        // Update clipboard in database
        upsert_clipboard_dto(decrypted_clipboard.clone()).await?;

        if let Some(provider) = &provider {
            if let Some(remote_clipboards) = &remote_clipboards
                .iter()
                .find(|c| c.id == decrypted_clipboard.clipboard.id)
            {
                provider
                    .update_clipboard(&decrypted_clipboard, &remote_clipboards)
                    .await
                    .expect("Failed to upsert clipboard");
            }
        }

        let progress = Progress {
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

pub fn decrypt_clipboard(
    mut clipboard: FullClipboardDto,
) -> Result<FullClipboardDto, EncryptionError> {
    if !clipboard.clipboard.encrypted {
        return Err(EncryptionError::NotEncrypted);
    }

    if let Some(text) = &mut clipboard.text {
        let decoded = STANDARD
            .decode(&text.data)
            .map_err(|_| EncryptionError::DecryptionFailed)?;
        let decrypted = decrypt_data(&decoded)?;
        text.data = String::from_utf8(decrypted).map_err(|_| EncryptionError::DecryptionFailed)?;
    }

    if let Some(html) = &mut clipboard.html {
        let decoded = STANDARD
            .decode(&html.data)
            .map_err(|_| EncryptionError::DecryptionFailed)?;
        let decrypted = decrypt_data(&decoded)?;
        html.data = String::from_utf8(decrypted).map_err(|_| EncryptionError::DecryptionFailed)?;
    }

    if let Some(rtf) = &mut clipboard.rtf {
        let decoded = STANDARD
            .decode(&rtf.data)
            .map_err(|_| EncryptionError::DecryptionFailed)?;
        let decrypted = decrypt_data(&decoded)?;
        rtf.data = String::from_utf8(decrypted).map_err(|_| EncryptionError::DecryptionFailed)?;
    }

    if let Some(image) = &mut clipboard.image {
        image.data = decrypt_data(&image.data)?;

        let thumbnail_decoded = STANDARD
            .decode(&image.thumbnail)
            .map_err(|_| EncryptionError::DecryptionFailed)?;
        let thumbnail_decrypted = decrypt_data(&thumbnail_decoded)?;
        image.thumbnail = STANDARD.encode(thumbnail_decrypted);
    }

    if !clipboard.files.is_empty() {
        for file in &mut clipboard.files {
            let name_decoded = STANDARD
                .decode(&file.name)
                .map_err(|_| EncryptionError::DecryptionFailed)?;
            let name_decrypted = decrypt_data(&name_decoded)?;
            file.name =
                String::from_utf8(name_decrypted).map_err(|_| EncryptionError::DecryptionFailed)?;

            file.data = decrypt_data(&file.data)?;

            if let Some(extension) = &file.extension {
                let ext_decoded = STANDARD
                    .decode(extension)
                    .map_err(|_| EncryptionError::DecryptionFailed)?;
                let ext_decrypted = decrypt_data(&ext_decoded)?;
                file.extension = Some(
                    String::from_utf8(ext_decrypted)
                        .map_err(|_| EncryptionError::DecryptionFailed)?,
                );
            }

            if let Some(mime_type) = &file.mime_type {
                let mime_decoded = STANDARD
                    .decode(mime_type)
                    .map_err(|_| EncryptionError::DecryptionFailed)?;
                let mime_decrypted = decrypt_data(&mime_decoded)?;
                file.mime_type = Some(
                    String::from_utf8(mime_decrypted)
                        .map_err(|_| EncryptionError::DecryptionFailed)?,
                );
            }
        }
    }

    clipboard.clipboard.encrypted = false;
    Ok(clipboard)
}

/// Decrypts data using AES-256-GCM
pub fn decrypt_data(encrypted_data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    if encrypted_data.len() < 12 {
        return Err(EncryptionError::NotEncrypted);
    }

    let key_bytes = ENCRYPTION_KEY
        .lock()
        .map_err(|_| EncryptionError::KeyLockFailed)?
        .ok_or(EncryptionError::NoKey)?;

    // Create unbound key from key bytes
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
        .map_err(|_| EncryptionError::DecryptionFailed)?;
    let key = aead::LessSafeKey::new(unbound_key);

    // Split nonce and encrypted data
    let nonce = aead::Nonce::assume_unique_for_key(
        encrypted_data[..12]
            .try_into()
            .map_err(|_| EncryptionError::DecryptionFailed)?,
    );

    // Decrypt data
    let mut in_out = encrypted_data[12..].to_vec();
    match key.open_in_place(nonce, aead::Aad::empty(), &mut in_out) {
        Ok(decrypted) => Ok(decrypted.to_vec()),
        Err(_) => {
            if looks_like_encrypted_data(encrypted_data) {
                Err(EncryptionError::InvalidKey)
            } else {
                printlog!("Data is not encrypted");
                Err(EncryptionError::NotEncrypted)
            }
        }
    }
}

pub async fn remove_encryption(password: String) -> Result<(), CommandError> {
    if !is_key_set() {
        return Err(CommandError::new("MAIN.ERROR.NO_ENCRYPTION_KEY_SET"));
    }

    let is_password_valid =
        verify_password(password).map_err(|e| CommandError::new(&e.to_string()))?;

    if !is_password_valid {
        return Err(CommandError::new("MAIN.ERROR.INCORRECT_PASSWORD"));
    }

    decrypt_all_clipboards().await?;

    let mut settings = get_global_settings();
    settings.encryption = false;
    update_settings_db(settings).await?;

    clear_encryption_key();

    Ok(())
}

pub fn clear_encryption_key() {
    *ENCRYPTION_KEY
        .lock()
        .map_err(|e| CommandError::new(&e.to_string()))
        .unwrap() = None;
}

pub fn verify_password(password: String) -> Result<bool, EncryptionError> {
    let mut hasher = ring::digest::Context::new(&ring::digest::SHA256);
    hasher.update(password.as_bytes());
    let key = hasher.finish();
    let mut provided_key = [0u8; 32];
    provided_key.copy_from_slice(key.as_ref());

    let current_key = ENCRYPTION_KEY
        .lock()
        .map_err(|_| EncryptionError::KeyLockFailed)?
        .ok_or(EncryptionError::NoKey)?;

    Ok(provided_key == current_key)
}

pub fn init_password_lock() {
    printlog!("Emitting password lock event");
    get_app()
        .emit_to(
            EventTarget::any(),
            ListenEvent::PasswordLock.to_string().as_str(),
            (),
        )
        .expect("Failed to emit download progress event");
}
