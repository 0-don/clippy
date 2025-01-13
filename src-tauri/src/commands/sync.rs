use crate::{
    service::{
        settings::update_settings_synchronize_db,
        sync::{sync_interval_toggle, SyncProvider},
    },
    utils::providers::google_drive::GoogleDriveProvider,
};
use common::{printlog, types::types::CommandError};
use std::sync::Arc;

#[tauri::command]
pub async fn sync_is_authenticated() -> Result<bool, CommandError> {
    let provider = Arc::new(
        GoogleDriveProvider::new()
            .await
            .expect("Failed to initialize sync provider"),
    );

    if provider.is_authenticated().await {
        printlog!("Authenticated");
        update_settings_synchronize_db(true).await?;
        Ok(provider
            .is_authenticated()
            .await
            .then(|| true)
            .unwrap_or(false))
    } else {
        printlog!("Authentication failed");
        update_settings_synchronize_db(false).await?;
        Err(CommandError::Error("Authentication failed".to_string()))
    }
}

#[tauri::command]
pub async fn sync_authenticate_toggle() -> Result<bool, CommandError> {
    let sync = sync_interval_toggle()
        .await
        .expect("Failed to toggle sync interval");
    Ok(sync)
}
