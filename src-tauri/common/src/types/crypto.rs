use std::{fmt, sync::Mutex};

// Global encryption key stored in memory
pub static ENCRYPTION_KEY: Mutex<Option<[u8; 32]>> = Mutex::new(None);

#[derive(Debug)]
pub enum EncryptionError {
    NoKey,
    InvalidKey,
    NotEncrypted,
    EncryptionFailed(String),
    DecryptionFailed(String),
}

impl std::error::Error for EncryptionError {}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EncryptionError::NoKey => write!(f, "No encryption key set"),
            EncryptionError::InvalidKey => write!(f, "Invalid encryption key"),
            EncryptionError::NotEncrypted => write!(f, "Data is not encrypted"),
            EncryptionError::EncryptionFailed(e) => write!(f, "Encryption failed: {}", e),
            EncryptionError::DecryptionFailed(e) => write!(f, "Decryption failed: {}", e),
        }
    }
}
