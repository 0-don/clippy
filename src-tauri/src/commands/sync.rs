use crate::{service::sync::SyncProvider, utils::providers::google_drive::GoogleDriveProvider};
use common::{printlog, types::types::CommandError};
use std::sync::Arc;

#[tauri::command]
pub async fn sync_is_authenticated() -> Result<String, CommandError> {
    let provider = Arc::new(
        GoogleDriveProvider::new()
            .await
            .expect("Failed to initialize sync provider"),
    );

    if provider.is_authenticated().await {
        printlog!("Authenticated");
        Ok(provider
            .is_authenticated()
            .await
            .then(|| format!("Authenticated"))
            .unwrap_or("Not authenticated".to_string()))
    } else {
        printlog!("Authentication failed");
        Err(CommandError::Error("Authentication failed".to_string()))
    }
}
