use super::{
    cipher::{clear_encryption_key, is_encryption_key_set, verify_encryption_password},
    clipboard::{load_clipboards_with_relations, upsert_clipboard_dto},
    settings::{get_global_settings, update_settings_db},
    sync::{get_sync_manager, get_sync_provider},
};
use crate::{
    prelude::*,
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
use rayon::prelude::*;
use ring::aead;
use sea_orm::prelude::Uuid;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use tauri::{Emitter, EventTarget};

/// Rows per page. Blobs (image/file data) dominate memory, so keep pages small to
/// cap RAM at one page of decrypted blobs regardless of total DB size.
const DECRYPT_PAGE_SIZE: u64 = 32;

/// Streams decryption of all locally-stored encrypted clipboards, paginated and
/// decrypted in parallel across cores. `on_batch(batch, current, total)` is called
/// once per page with the trimmed, decrypted clipboards (heavy blobs stripped) so the
/// caller can stream them to the UI or drive a progress bar.
pub async fn decrypt_all_clipboards_streaming<F>(mut on_batch: F) -> Result<(), CommandError>
where
    F: FnMut(Vec<FullClipboardDto>, usize, usize),
{
    let settings = get_global_settings();
    let db = db();

    // Read the key once: workers reuse this copy, no per-field mutex contention.
    let key = read_encryption_key().map_err(|e| CommandError::new(&e.to_string()))?;

    // Pull remote encrypted rows into the local DB first so pagination sees everything.
    let (provider, remote_clipboards) = if settings.sync {
        let provider = get_sync_provider().await;
        let remote_clipboards = provider
            .fetch_all_clipboards()
            .await
            .expect("Failed to fetch remote clipboards");

        let download_total = remote_clipboards.len();
        for (index, remote) in remote_clipboards.iter().enumerate() {
            get_app()
                .emit_to(
                    EventTarget::any(),
                    ListenEvent::Progress.to_string().as_str(),
                    Progress {
                        label: "SETTINGS.ENCRYPT.DOWNLOADING_REMOTE_CLIPBOARDS".to_string(),
                        total: download_total,
                        current: index + 1,
                    },
                )
                .map_err(|e| CommandError::new(&e.to_string()))?;

            if !remote.encrypted || remote.deleted_at.is_some() {
                continue;
            }

            // Already present locally: pagination will pick it up.
            if clipboard::Entity::find_by_id(remote.id)
                .one(db)
                .await?
                .is_some()
            {
                continue;
            }

            // Bind to Option (dropping the non-Send error) before the next await so the
            // command future stays Send.
            let downloaded = provider.download_by_id(&remote.provider_id).await.ok();
            if let Some(clipboard) = downloaded {
                upsert_clipboard_dto(clipboard).await?;
            }
        }
        (Some(provider), remote_clipboards)
    } else {
        (None, Vec::new())
    };

    let total = clipboard::Entity::find()
        .filter(clipboard::Column::Encrypted.eq(true))
        .count(db)
        .await? as usize;

    // UUIDv7 ids are time-ordered, so order_by_desc(Id) is a stable newest-first cursor.
    let mut paginator = clipboard::Entity::find()
        .filter(clipboard::Column::Encrypted.eq(true))
        .order_by_desc(clipboard::Column::Id)
        .paginate(db, DECRYPT_PAGE_SIZE);

    let mut processed = 0usize;

    while let Some(models) = paginator.fetch_and_next().await? {
        let page = load_clipboards_with_relations(models).await;

        // Decrypt the whole page in parallel; pure CPU, no DB, no global lock.
        let decrypted: Vec<Result<FullClipboardDto, (Uuid, EncryptionError)>> =
            tokio::task::spawn_blocking(move || {
                page.into_par_iter()
                    .map(|c| {
                        let id = c.clipboard.id;
                        decrypt_clipboard_with_key(c, &key).map_err(|e| (id, e))
                    })
                    .collect()
            })
            .await
            .map_err(|e| CommandError::new(&e.to_string()))?;

        // Serial DB writes (SQLite is single-writer) + collect this page's successes.
        let mut batch: Vec<FullClipboardDto> = Vec::with_capacity(decrypted.len());
        for result in decrypted {
            processed += 1;
            match result {
                Ok(clipboard) => {
                    upsert_clipboard_dto(clipboard.clone()).await?;

                    if let Some(provider) = &provider {
                        if let Some(remote) =
                            remote_clipboards.iter().find(|r| r.id == clipboard.clipboard.id)
                        {
                            provider.update_clipboard(&clipboard, remote).await.ok();
                        }
                    }
                    batch.push(clipboard);
                }
                Err((id, e)) => {
                    printlog!("Failed to decrypt clipboard {}: {:?}", id, e);

                    clipboard::Entity::delete_by_id(id).exec(db).await?;

                    if let Some(provider) = &provider {
                        if let Some(remote) = remote_clipboards.iter().find(|r| r.id == id) {
                            provider.mark_for_deletion(remote).await;
                        }
                    }
                }
            }
        }

        on_batch(common::io::clipboard::trim_clipboard_data(batch), processed, total);
    }

    Ok(())
}

/// Non-streaming wrapper for callers (remove-encryption, sync) that only need the
/// existing Progress event. Drives the same paginated parallel core.
pub async fn decrypt_all_clipboards() -> Result<(), CommandError> {
    let mut emit_err: Option<CommandError> = None;

    decrypt_all_clipboards_streaming(|_batch, current, total| {
        if emit_err.is_some() {
            return;
        }
        if let Err(e) = get_app().emit_to(
            EventTarget::any(),
            ListenEvent::Progress.to_string().as_str(),
            Progress {
                label: "SETTINGS.ENCRYPT.DECRYPTION_PROGRESS_LOCAL".to_string(),
                total,
                current,
            },
        ) {
            emit_err = Some(CommandError::new(&e.to_string()));
        }
    })
    .await?;

    if let Some(e) = emit_err {
        return Err(e);
    }

    Ok(())
}

/// Decrypts a base64-encoded encrypted string field
fn decrypt_string_field(
    data: &str,
    clipboard_id: Uuid,
    field_name: &str,
    key: &[u8; 32],
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
    let decrypted = decrypt_data_with_key(&decoded, key).map_err(|e| {
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
    key: &[u8; 32],
) -> Result<Vec<u8>, EncryptionError> {
    decrypt_data_with_key(data, key).map_err(|e| {
        printlog!(
            "Failed to decrypt {} for clipboard {}: {:?}",
            field_name,
            clipboard_id,
            e
        );
        e
    })
}

/// Reads the global encryption key once and delegates to the key-parametrized decrypt.
pub fn decrypt_clipboard(
    clipboard: FullClipboardDto,
) -> Result<FullClipboardDto, EncryptionError> {
    let key = read_encryption_key()?;
    decrypt_clipboard_with_key(clipboard, &key)
}

/// Decrypts a clipboard using an explicit key. Pure CPU, no global lock: safe to run
/// across rayon workers when decrypting many clipboards in parallel.
pub fn decrypt_clipboard_with_key(
    mut clipboard: FullClipboardDto,
    key: &[u8; 32],
) -> Result<FullClipboardDto, EncryptionError> {
    if !clipboard.clipboard.encrypted {
        return Err(EncryptionError::NotEncrypted);
    }
    let id = clipboard.clipboard.id;

    if let Some(text) = &mut clipboard.text {
        text.data = decrypt_string_field(&text.data, id, "text", key)?;
    }

    if let Some(html) = &mut clipboard.html {
        html.data = decrypt_string_field(&html.data, id, "html", key)?;
    }

    if let Some(rtf) = &mut clipboard.rtf {
        rtf.data = decrypt_string_field(&rtf.data, id, "rtf", key)?;
    }

    if let Some(image) = &mut clipboard.image {
        image.data = decrypt_binary_field(&image.data, id, "image", key)?;

        let thumb_decoded = STANDARD.decode(&image.thumbnail).map_err(|e| {
            printlog!(
                "Failed to base64 decode thumbnail for clipboard {}: {}",
                id,
                e
            );
            EncryptionError::DecryptionFailed
        })?;
        image.thumbnail =
            STANDARD.encode(decrypt_binary_field(&thumb_decoded, id, "thumbnail", key)?);

        if let Some(ocr_text) = &image.ocr_text {
            // OCR text might not be encrypted (e.g. added after encryption was enabled)
            if let Ok(decoded) = STANDARD.decode(ocr_text) {
                if let Ok(decrypted) = decrypt_data_with_key(&decoded, key) {
                    if let Ok(str_data) = String::from_utf8(decrypted) {
                        image.ocr_text = Some(str_data);
                    }
                }
            }
        }
    }

    for (i, file) in clipboard.files.iter_mut().enumerate() {
        file.name = decrypt_string_field(&file.name, id, &format!("file[{}].name", i), key)?;
        file.data = decrypt_binary_field(&file.data, id, &format!("file[{}].data", i), key)?;

        if let Some(ext) = &file.extension {
            file.extension =
                Some(decrypt_string_field(ext, id, &format!("file[{}].ext", i), key)?);
        }
        if let Some(mime) = &file.mime_type {
            file.mime_type =
                Some(decrypt_string_field(mime, id, &format!("file[{}].mime", i), key)?);
        }
    }

    clipboard.clipboard.encrypted = false;
    Ok(clipboard)
}

/// Reads the global encryption key bytes once. Callers that decrypt many items should
/// call this once and reuse the copy, avoiding per-field mutex contention.
pub fn read_encryption_key() -> Result<[u8; 32], EncryptionError> {
    let guard = ENCRYPTION_KEY
        .lock()
        .map_err(|_| EncryptionError::KeyLockFailed)?;
    let key_data = guard.as_ref().ok_or(EncryptionError::NoKey)?;
    Ok(key_data.0)
}

/// Decrypts data using AES-256-GCM with an explicit key. No global lock: safe for
/// parallel decrypt across rayon workers.
pub fn decrypt_data_with_key(
    encrypted_data: &[u8],
    key: &[u8; 32],
) -> Result<Vec<u8>, EncryptionError> {
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

    // Create unbound key from key bytes
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key)
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
    remove_encryption_streaming(password, |_batch, current, total| {
        get_app()
            .emit_to(
                EventTarget::any(),
                ListenEvent::Progress.to_string().as_str(),
                Progress {
                    label: "SETTINGS.ENCRYPT.DECRYPTION_PROGRESS_LOCAL".to_string(),
                    total,
                    current,
                },
            )
            .ok();
    })
    .await
}

/// Permanently removes encryption, streaming each decrypted page to `on_batch` so the
/// settings screen can show live progress and fill the list as data decrypts.
pub async fn remove_encryption_streaming<F>(
    password: String,
    on_batch: F,
) -> Result<(), CommandError>
where
    F: FnMut(Vec<FullClipboardDto>, usize, usize),
{
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
    decrypt_all_clipboards_streaming(on_batch).await?;

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
