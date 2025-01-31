use super::{
    clipboard::load_clipboards_with_relations,
    decrypt::{decrypt_all_clipboards, decrypt_clipboard},
    encrypt::encrypt_all_clipboards,
    settings::get_global_settings,
};
use crate::tao::{connection::db, global::get_app};
use crate::{prelude::*, service::settings::update_settings_db};
use common::types::{
    cipher::{EncryptionError, ENCRYPTION_KEY},
    enums::{ListenEvent, PasswordAction},
    types::CommandError,
};
use entity::clipboard;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tauri::{Emitter, EventTarget};

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
            let db = db().await?;
            let encrypted_clipboard = clipboard::Entity::find()
                .filter(clipboard::Column::Encrypted.eq(true))
                .one(&db)
                .await?;

            if let Some(clipboard) = encrypted_clipboard {
                let mut clipboards = load_clipboards_with_relations(vec![clipboard]).await;
                decrypt_clipboard(clipboards.remove(0)).map_err(|e| {
                    clear_encryption_key();
                    match e {
                        EncryptionError::DecryptionFailed => {
                            CommandError::new("MAIN.ERROR.INCORRECT_PASSWORD")
                        }
                        _ => CommandError::new(&e.to_string()),
                    }
                })?;

                // Only proceed with full decryption for sync decrypt
                if matches!(action, PasswordAction::SyncDecrypt) {
                    let mut settings = get_global_settings();
                    settings.encryption = false;
                    update_settings_db(settings).await?;
                    decrypt_all_clipboards().await?;
                } else {
                    // encrypt all clipboards again if new were added before password was set
                    encrypt_all_clipboards(false).await?;
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
