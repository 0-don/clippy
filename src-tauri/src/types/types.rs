use global_hotkey::hotkey::HotKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub db: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPath {
    pub config_path: String,
    pub db_file_path: String,
    pub config_file_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseInfo {
    pub records: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct Key {
    pub id: u32,
    pub global: bool,
    pub key_str: String,
    pub event: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub key: String,
    pub hotkey: HotKey,
}
