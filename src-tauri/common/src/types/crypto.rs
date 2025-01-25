use std::{fmt, sync::Mutex};

// Global encryption key stored in memory
pub static ENCRYPTION_KEY: Mutex<Option<[u8; 32]>> = Mutex::new(None);

#[derive(Debug)]
pub enum EncryptionError {
    NoKey,
    KeyLockFailed,
    InvalidKey,
    NotEncrypted,
    EncryptionFailed,
    DecryptionFailed,
}

impl std::error::Error for EncryptionError {}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EncryptionError::NoKey => write!(f, "NO_ENCRYPTION_KEY_SET"),
            EncryptionError::KeyLockFailed => write!(f, "MAIN.ERROR.ENCRYPTION_KEY_LOCK_FAILED"),
            EncryptionError::InvalidKey => write!(f, "MAIN.ERROR.INVALID_ENCRYPTION_KEY"),
            EncryptionError::NotEncrypted => write!(f, "MAIN.ERROR.DATA_IS_NOT_ENCRYPTED"),
            EncryptionError::EncryptionFailed => write!(f, "MAIN.ERROR.ENCRYPTION_FAILED"),
            EncryptionError::DecryptionFailed => write!(f, "MAIN.ERROR.DECRYPTION_FAILED"),
        }
    }
}
