use super::encrypt::looks_like_encrypted_data;
use common::types::crypto::{EncryptionError, ENCRYPTION_KEY};
use ring::aead;

pub async fn decrypt_all_clipboards() -> Result<(), EncryptionError> {
    // let db = db().await?;

    // let clipboards =
    //     load_clipboards_with_relations(clipboard::Entity::find().all(&db).await?).await;

    // for clipboard in clipboards {
    //     let decrypted_data = decrypt_data(&clipboard.data)?;
    //     let clipboard = clipboard::Entity::update(clipboard.reset_data(decrypted_data))
    //         .exec(&db)
    //         .await?;
    // }

    Ok(())
}

/// Decrypts data using AES-256-GCM
pub fn decrypt_data(encrypted_data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    if encrypted_data.len() < 12 {
        return Err(EncryptionError::NotEncrypted);
    }

    let key_bytes = ENCRYPTION_KEY
        .lock()
        .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?
        .ok_or(EncryptionError::NoKey)?;

    // Create unbound key from key bytes
    let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
        .map_err(|_| EncryptionError::DecryptionFailed("Failed to create key".to_string()))?;
    let key = aead::LessSafeKey::new(unbound_key);

    // Split nonce and encrypted data
    let nonce = aead::Nonce::assume_unique_for_key(
        encrypted_data[..12]
            .try_into()
            .map_err(|_| EncryptionError::DecryptionFailed("Invalid nonce".to_string()))?,
    );

    // Decrypt data
    let mut in_out = encrypted_data[12..].to_vec();
    match key.open_in_place(nonce, aead::Aad::empty(), &mut in_out) {
        Ok(decrypted) => Ok(decrypted.to_vec()),
        Err(_) => {
            if looks_like_encrypted_data(encrypted_data) {
                Err(EncryptionError::InvalidKey)
            } else {
                Err(EncryptionError::NotEncrypted)
            }
        }
    }
}

pub fn verify_password(password: String) -> Result<bool, EncryptionError> {
    let mut hasher = ring::digest::Context::new(&ring::digest::SHA256);
    hasher.update(password.as_bytes());
    let key = hasher.finish();
    let mut provided_key = [0u8; 32];
    provided_key.copy_from_slice(key.as_ref());

    let current_key = ENCRYPTION_KEY
        .lock()
        .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?
        .ok_or(EncryptionError::NoKey)?;

    Ok(provided_key == current_key)
}
