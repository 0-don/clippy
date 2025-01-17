use crate::service::{
    settings::{get_settings_db, update_settings_synchronize_db},
    sync::{get_sync_provider, sync_interval_toggle},
};
use common::{printlog, types::types::CommandError};
use entity::settings::{self, ActiveModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use tao::connection::db;

#[tauri::command]
pub async fn sync_is_authenticated() -> Result<bool, CommandError> {
    let provider = get_sync_provider().await;

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
    Ok(sync_interval_toggle()
        .await
        .expect("Failed to toggle sync interval"))
}

#[tauri::command]
pub async fn sync_limit_change(sync_limit: i32) -> Result<settings::Model, CommandError> {
    let db: DatabaseConnection = db().await?;
    let mut settings = get_settings_db().await?;

    settings.sync_limit = sync_limit;

    let active_model: ActiveModel = settings.into();

    let _ = settings::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    Ok(get_settings_db().await?)
}
