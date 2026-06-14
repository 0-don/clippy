use crate::service::{
    cipher::{
        handle_password_unlock, handle_password_unlock_streaming, is_encryption_key_set,
        set_encryption_key,
    },
    decrypt::{remove_encryption, remove_encryption_streaming},
    encrypt::encrypt_all_clipboards,
    settings::{get_global_settings, update_settings_db},
};
use common::types::{
    enums::PasswordAction, orm_query::FullClipboardDto, types::CommandError,
};
use serde::Serialize;
use tauri::ipc::Channel;

/// Streamed decryption events, mirroring SearchEvent. `current`/`total` drive progress;
/// `clipboards` are trimmed (heavy blobs stripped) for cheap IPC and live list fill.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum DecryptEvent {
    Batch {
        clipboards: Vec<FullClipboardDto>,
        current: usize,
        total: usize,
    },
    Done {},
}

#[tauri::command]
pub async fn password_unlock(password: String, action: PasswordAction) -> Result<(), CommandError> {
    handle_password_unlock(password, action).await
}

/// Streaming permanent-decrypt unlock (SyncDecrypt). Sends a Batch per decrypted page,
/// then Done. The frontend appends each batch to the live clipboard list.
#[tauri::command]
pub async fn password_unlock_stream(
    password: String,
    on_chunk: Channel<DecryptEvent>,
) -> Result<(), CommandError> {
    let sink = on_chunk.clone();
    handle_password_unlock_streaming(password, move |clipboards, current, total| {
        sink.send(DecryptEvent::Batch {
            clipboards,
            current,
            total,
        })
        .ok();
    })
    .await?;

    on_chunk
        .send(DecryptEvent::Done {})
        .map_err(|e| CommandError::new(&e.to_string()))?;
    Ok(())
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
    remove_encryption(password).await
}

/// Streaming disable-encryption for the settings screen: live progress + list fill.
#[tauri::command]
pub async fn disable_encryption_stream(
    password: String,
    on_chunk: Channel<DecryptEvent>,
) -> Result<(), CommandError> {
    let sink = on_chunk.clone();
    remove_encryption_streaming(password, move |clipboards, current, total| {
        sink.send(DecryptEvent::Batch {
            clipboards,
            current,
            total,
        })
        .ok();
    })
    .await?;

    on_chunk
        .send(DecryptEvent::Done {})
        .map_err(|e| CommandError::new(&e.to_string()))?;
    Ok(())
}
