use crate::{
    service::{
        clipboard::load_clipboards_with_relations,
        decrypt::{clear_encryption_key, decrypt_clipboard, remove_encryption},
        encrypt::{encrypt_all_clipboards, is_key_set, set_encryption_key},
        settings::{get_global_settings, update_settings_db},
    },
    tao::connection::db,
};
use common::types::{cipher::EncryptionError, types::CommandError};
use entity::clipboard;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

#[tauri::command]
pub async fn password_unlock(password: String) -> Result<(), CommandError> {
    if is_key_set() {
        return Err(CommandError::new("MAIN.ERROR.ENCRYPTION_KEY_ALREADY_SET"));
    }

    set_encryption_key(&password).map_err(|e| CommandError::new(&e.to_string()))?;

    let db = db().await?;
    let encrypted_clipboard = clipboard::Entity::find()
        .filter(clipboard::Column::Encrypted.eq(true))
        .one(&db)
        .await?;

    if let Some(clipboard) = encrypted_clipboard {
        let clipboards = load_clipboards_with_relations(vec![clipboard]).await;
        decrypt_clipboard(clipboards[0].clone()).map_err(|e| {
            clear_encryption_key();
            match e {
                EncryptionError::DecryptionFailed => {
                    CommandError::new("MAIN.ERROR.INCORRECT_PASSWORD")
                }
                _ => CommandError::new(&e.to_string()),
            }
        })?;

        encrypt_all_clipboards(false).await?;
    } else {
        remove_encryption(password).await?;
    }

    Ok(())
}

#[tauri::command]
pub async fn enable_encryption(
    password: String,
    confirm_password: String,
) -> Result<(), CommandError> {
    if is_key_set() {
        return Err(CommandError::new("MAIN.ERROR.ENCRYPTION_KEY_ALREADY_SET"));
    }

    if password != confirm_password {
        return Err(CommandError::new("MAIN.ERROR.PASSWORD_NOT_MATCH"));
    }

    set_encryption_key(&password).map_err(|e| CommandError::new(&e.to_string()))?;

    encrypt_all_clipboards(true).await?;

    let mut settings = get_global_settings();
    settings.encryption = true;
    update_settings_db(settings).await?;

    Ok(())
}

#[tauri::command]
pub async fn disable_encryption(password: String) -> Result<(), CommandError> {
    match remove_encryption(password).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
