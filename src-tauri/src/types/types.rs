use global_hotkey::hotkey::HotKey;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use tauri_plugin_shell::Error as ShellError;

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
#[allow(dead_code)]
pub struct Key {
    pub id: u32,
    pub state: bool,
    pub is_global: bool,
    pub key_str: String,
    pub event: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub key: String,
    pub hotkey: HotKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowName {
    About,
    Settings,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandError {
    Io(String),
    Json(String),
    DbErr(String),
    Shell(String),
    Opener(String),
    Image(String),
    Tauri(String),
    Option(String),
}

impl From<std::io::Error> for CommandError {
    fn from(err: std::io::Error) -> Self {
        CommandError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for CommandError {
    fn from(err: serde_json::Error) -> Self {
        CommandError::Json(err.to_string())
    }
}

impl From<DbErr> for CommandError {
    fn from(err: DbErr) -> Self {
        CommandError::DbErr(err.to_string())
    }
}

impl From<ShellError> for CommandError {
    fn from(err: ShellError) -> Self {
        CommandError::Shell(err.to_string())
    }
}

impl From<tauri_plugin_opener::Error> for CommandError {
    fn from(err: tauri_plugin_opener::Error) -> Self {
        CommandError::Opener(err.to_string())
    }
}

impl From<image::ImageError> for CommandError {
    fn from(err: image::ImageError) -> Self {
        CommandError::Image(err.to_string())
    }
}

impl From<tauri::Error> for CommandError {
    fn from(err: tauri::Error) -> Self {
        CommandError::Tauri(err.to_string())
    }
}
