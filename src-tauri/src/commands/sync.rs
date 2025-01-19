use crate::service::{
    settings::get_settings_db,
    sync::{get_sync_provider, sync_interval_toggle},
};
use common::types::types::CommandError;
use entity::settings::{self, ActiveModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use tao::connection::db;

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

    let settings = settings::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    if settings.sync {
        tauri::async_runtime::spawn(async {
            let provider = get_sync_provider().await;
            let remote_clipboards = provider
                .fetch_all_clipboards()
                .await
                .expect("Failed to fetch all clipboards");
            provider
                .cleanup_old_clipboards(&remote_clipboards)
                .await
                .expect("Failed to cleanup old clipboards");
        });
    }

    Ok(get_settings_db().await?)
}
