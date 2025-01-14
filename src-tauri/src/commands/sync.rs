use crate::service::{
    settings::update_settings_synchronize_db,
    sync::{get_sync_provider, sync_interval_toggle},
};
use common::{printlog, types::types::CommandError};

#[tauri::command]
pub async fn sync_is_authenticated() -> Result<bool, CommandError> {
    let provider = get_sync_provider().await?;

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
