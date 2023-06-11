use entity::hotkey::{self, Model};
use sea_orm::EntityTrait;
use sea_orm::{DatabaseConnection, DbErr};

use crate::connection;

#[tauri::command]
pub async fn get_hotkeys() -> Result<Vec<Model>, String> {
    let res = get_all_hotkeys().await;

    Ok(res.unwrap())
}

async fn get_all_hotkeys() -> Result<Vec<Model>, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let hotkeys = hotkey::Entity::find().all(&db).await?;

    Ok(hotkeys)
}
