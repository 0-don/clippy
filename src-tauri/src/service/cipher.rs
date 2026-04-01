use super::{
    clipboard::load_clipboards_with_relations,
    decrypt::{decrypt_all_clipboards, decrypt_clipboard},
    encrypt::encrypt_all_clipboards,
    settings::get_global_settings,
};
use crate::tao::{connection::db, global::get_app};
use crate::{prelude::*, service::settings::update_settings_db};
use common::types::{
    cipher::{EncryptionError, EncryptionKeyData, ENCRYPTION_KEY},
    enums::{ListenEvent, PasswordAction},
    types::CommandError,
};
use entity::clipboard;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::num::NonZeroU32;
use tauri::{Emitter, EventTarget};

const PBKDF2_SALT: &[u8] = b"clippy";
const PBKDF2_ITERATIONS: u32 = 600_000;

pub async fn handle_password_unlock(
    password: String,
    action: PasswordAction,
) -> Result<(), CommandError> {
    printlog!("action: {:?}", action);

    match action {
        PasswordAction::Encrypt => {
            set_encryption_key(&password).map_err(|e| CommandError::new(&e.to_string()))?;
            encrypt_all_clipboards(false).await?;
            let mut settings = get_global_settings();
            settings.encryption = true;
            update_settings_db(settings).await?;
        }
        PasswordAction::Decrypt | PasswordAction::SyncDecrypt => {
            set_encryption_key(&password).map_err(|e| CommandError::new(&e.to_string()))?;

            // Verify password by trying to decrypt something
            let db = db();
            let encrypted_clipboard = clipboard::Entity::find()
                .filter(clipboard::Column::Encrypted.eq(true))
                .one(db)
                .await?;

            if let Some(clipboard) = encrypted_clipboard {
                let mut clipboards = load_clipboards_with_relations(vec![clipboard]).await;
                let test_clipboard = clipboards.remove(0);
                let mut used_legacy = false;

                // Try PBKDF2 key first, fall back to legacy SHA-256 for migration
                if decrypt_clipboard(test_clipboard.clone()).is_err() {
                    clear_encryption_key();
                    set_encryption_key_legacy_sha256(&password)
                        .map_err(|e| CommandError::new(&e.to_string()))?;

                    decrypt_clipboard(test_clipboard).map_err(|e| {
                        clear_encryption_key();
                        match e {
                            EncryptionError::DecryptionFailed | EncryptionError::InvalidKey => {
                                CommandError::new("MAIN.ERROR.INCORRECT_PASSWORD")
                            }
                            _ => CommandError::new(&e.to_string()),
                        }
                    })?;
                    used_legacy = true;
                }

                if used_legacy {
                    // Legacy SHA-256 key worked: keep using it for this session.
                    // Existing data stays encrypted with the old key.
                    // New clipboards will also use the legacy key since it's what's set.
                    if matches!(action, PasswordAction::SyncDecrypt) {
                        let mut settings = get_global_settings();
                        settings.encryption = false;
                        update_settings_db(settings).await?;
                        decrypt_all_clipboards().await?;
                    } else {
                        encrypt_all_clipboards(false).await?;
                    }
                } else {
                    // Normal path (PBKDF2 key worked)
                    if matches!(action, PasswordAction::SyncDecrypt) {
                        let mut settings = get_global_settings();
                        settings.encryption = false;
                        update_settings_db(settings).await?;
                        decrypt_all_clipboards().await?;
                    } else {
                        encrypt_all_clipboards(false).await?;
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn clear_encryption_key() {
    *ENCRYPTION_KEY
        .lock()
        .map_err(|e| CommandError::new(&e.to_string()))
        .unwrap() = None;
}

/// Checks if encryption key is set
pub fn is_encryption_key_set() -> bool {
    ENCRYPTION_KEY.lock().map(|k| k.is_some()).unwrap_or(false)
}

pub fn verify_encryption_password(password: String) -> Result<bool, EncryptionError> {
    let mut provided_key = [0u8; 32];
    ring::pbkdf2::derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
        PBKDF2_SALT,
        password.as_bytes(),
        &mut provided_key,
    );

    let current_key = ENCRYPTION_KEY
        .lock()
        .map_err(|_| EncryptionError::KeyLockFailed)?;
    let current = current_key.as_ref().ok_or(EncryptionError::NoKey)?;

    // Constant-time comparison to prevent timing attacks
    let mut diff = 0u8;
    for (a, b) in provided_key.iter().zip(current.0.iter()) {
        diff |= a ^ b;
    }
    Ok(diff == 0)
}

/// Sets the encryption key derived from a password using PBKDF2
pub fn set_encryption_key(password: &str) -> Result<(), EncryptionError> {
    let mut key_bytes = [0u8; 32];
    ring::pbkdf2::derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(PBKDF2_ITERATIONS).unwrap(),
        PBKDF2_SALT,
        password.as_bytes(),
        &mut key_bytes,
    );

    *ENCRYPTION_KEY
        .lock()
        .map_err(|_| EncryptionError::KeyLockFailed)? = Some(EncryptionKeyData(key_bytes));

    Ok(())
}

/// Legacy SHA-256 key derivation for migrating existing encrypted users
pub fn set_encryption_key_legacy_sha256(password: &str) -> Result<(), EncryptionError> {
    let mut hasher = ring::digest::Context::new(&ring::digest::SHA256);
    hasher.update(password.as_bytes());
    let key = hasher.finish();
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(key.as_ref());

    *ENCRYPTION_KEY
        .lock()
        .map_err(|_| EncryptionError::KeyLockFailed)? = Some(EncryptionKeyData(key_bytes));

    Ok(())
}

pub fn init_password_lock_event(action: PasswordAction) {
    get_app()
        .emit_to(
            EventTarget::any(),
            ListenEvent::PasswordLock.to_string().as_str(),
            action,
        )
        .expect("Failed to emit password lock event");
}

pub fn init_encryption_password_lock() {
    let settings = get_global_settings();
    if !is_encryption_key_set() && settings.encryption {
        // When initializing, if encryption is enabled but no key is set,
        // we need to decrypt first before potentially following remote state
        init_password_lock_event(PasswordAction::Decrypt);
    }
}
