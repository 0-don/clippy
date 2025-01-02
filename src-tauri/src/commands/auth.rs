use crate::utils::google_drive_manager::DriveManager;
use common::{printlog, types::types::CommandError};

#[tauri::command]
pub async fn auth_google_drive() -> Result<String, CommandError> {
    let drive_manager = DriveManager::new().await?;

    if drive_manager.is_authenticated().await {
        printlog!("Authenticated");
        Ok(drive_manager
            .is_authenticated()
            .await
            .then(|| format!("Authenticated"))
            .unwrap_or("Not authenticated".to_string()))
    } else {
        printlog!("Authentication failed");
        Err(CommandError::Error("Authentication failed".to_string()))
    }
}
