use common::types::types::CommandError;

use crate::utils::google_drive_manager::DriveManager;

#[tauri::command]
pub async fn auth_google_drive() -> Result<(), CommandError> {
    let drive_manager = DriveManager::new().await?;

    if drive_manager.is_authenticated().await {
        Ok(())
    } else {
        Err(CommandError::Error("Authentication failed".to_string()))
    }
}
