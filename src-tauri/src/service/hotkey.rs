use entity::hotkey::{self, Model};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use crate::connection;

pub async fn get_all_hotkeys_db() -> Result<Vec<Model>, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let hotkeys = hotkey::Entity::find().all(&db).await?;

    Ok(hotkeys)
}
