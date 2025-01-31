use crate::service::{
    cipher::{handle_password_unlock, is_encryption_key_set, set_encryption_key},
    decrypt::remove_encryption,
    encrypt::encrypt_all_clipboards,
    settings::{get_global_settings, update_settings_db},
};
use common::types::{enums::PasswordAction, types::CommandError};

#[tauri::command]
pub async fn password_unlock(password: String, action: PasswordAction) -> Result<(), CommandError> {
    handle_password_unlock(password, action).await
}

#[tauri::command]
pub async fn enable_encryption(
    password: String,
    confirm_password: String,
) -> Result<(), CommandError> {
    if is_encryption_key_set() {
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
