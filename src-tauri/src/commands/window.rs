use crate::service::{
    clipboard::count_clipboards_db, settings::get_data_path, window::open_window,
};
use common::types::{
    enums::{FolderLocation, WebWindow},
    types::{CommandError, Config, DatabaseInfo},
};
use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub async fn open_new_window(window_name: WebWindow, title: Option<String>) {
    open_window(window_name, title).await;
}

#[tauri::command]
pub fn exit_app() {
    std::process::exit(0);
}

#[tauri::command]
pub fn get_app_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}

#[tauri::command]
pub fn open_browser_url(url: String, app: AppHandle) -> Result<(), CommandError> {
    Ok(app.opener().open_url(url, None::<String>)?)
}

#[tauri::command]
pub async fn get_db_info() -> Result<DatabaseInfo, CommandError> {
    let data_path = get_data_path();

    let config: Config = serde_json::from_str(&read_to_string(&data_path.config_file_path)?)?;
    let size = fs::metadata(config.db)?.len();

    let records = count_clipboards_db().await?;

    Ok(DatabaseInfo { records, size })
}

#[tauri::command]
pub async fn get_db_path() -> Result<String, CommandError> {
    let data_path = get_data_path();
    let config: Config = serde_json::from_str(&read_to_string(&data_path.config_file_path)?)?;
    Ok(config.db)
}

#[tauri::command]
pub async fn get_config_path() -> Result<String, CommandError> {
    let data_path = get_data_path();
    Ok(data_path.config_file_path)
}

#[tauri::command]
pub async fn open_folder(location: FolderLocation) -> Result<(), CommandError> {
    let data_path = get_data_path();

    let path = match location {
        FolderLocation::Database => {
            // Get the database path from config
            let config: Config =
                serde_json::from_str(&read_to_string(&data_path.config_file_path)?)?;
            PathBuf::from(&config.db)
                .parent()
                .map(|p| p.to_path_buf())
                .ok_or_else(|| {
                    CommandError::Error("Could not get database directory".to_string())
                })?
        }
        FolderLocation::Config => PathBuf::from(&data_path.config_path),
    };

    if !path.exists() {
        return Err(CommandError::Error("Path does not exist".to_string()));
    }

    if !path.is_dir() {
        return Err(CommandError::Error("Path is not a directory".to_string()));
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer").arg(path).spawn()?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(path).spawn()?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(path).spawn()?;
    }

    Ok(())
}
