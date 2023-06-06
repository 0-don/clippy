use entity::settings::{self, Model};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use crate::connection;

#[tauri::command]
pub async fn get_settings() -> Result<Model, String> {
    let res = get_settings_db().await;

    // Ok(res.unwrap())
    Ok(res.unwrap())
}

async fn get_settings_db() -> Result<Model, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let settings = settings::Entity::find_by_id(1).one(&db).await?;

    Ok(settings.unwrap())
}
