use crate::service::{
    decrypt::{decrypt_all_clipboards, verify_password},
    encrypt::{encrypt_all_clipboards, is_key_set, set_encryption_key},
    settings::{get_global_settings, update_settings_db},
};
use common::types::{crypto::ENCRYPTION_KEY, types::CommandError};

#[tauri::command]
pub async fn load_encryption_key(password: String) -> Result<(), CommandError> {
    if is_key_set() {
        return Err(CommandError::new("MAIN.ERROR.ENCRYPTION_KEY_ALREADY_SET"));
    }

    set_encryption_key(&password).map_err(|e| CommandError::new(&e.to_string()))?;

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

    encrypt_all_clipboards().await?;

    let mut settings = get_global_settings();
    settings.encryption = true;
    update_settings_db(settings).await?;

    Ok(())
}

#[tauri::command]
pub async fn disable_encryption(password: String) -> Result<(), CommandError> {
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

    *ENCRYPTION_KEY
        .lock()
        .map_err(|e| CommandError::new(&e.to_string()))? = None;

    Ok(())
}
