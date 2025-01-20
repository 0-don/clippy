use crate::service::{
    settings::get_settings_db,
    sync::{get_sync_provider, sync_toggle, upsert_settings_sync},
};
use common::types::types::CommandError;
use entity::settings::{self, ActiveModel};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};
use tao::connection::db;

#[tauri::command]
pub async fn sync_authenticate_toggle() -> Result<bool, CommandError> {
    Ok(sync_toggle().await?)
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
        tauri::async_runtime::spawn(async move {
            let provider = get_sync_provider().await;
            let remote_clipboards = provider
                .fetch_all_clipboards()
                .await
                .expect("Failed to fetch all clipboards");
            provider
                .cleanup_old_clipboards(&remote_clipboards)
                .await
                .expect("Failed to cleanup old clipboards");

            upsert_settings_sync(&settings).expect("Failed to upsert settings");
        });
    }

    Ok(get_settings_db().await?)
}
