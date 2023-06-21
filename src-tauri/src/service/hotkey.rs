use entity::hotkey::{self, Model, ActiveModel};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ActiveModelTrait};

use crate::connection;

pub async fn get_all_hotkeys_db() -> Result<Vec<Model>, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let hotkeys = hotkey::Entity::find().all(&db).await?;

    Ok(hotkeys)
}


pub async fn update_hotkey_db(hotkey: Model) -> Result<Model, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let active_model: ActiveModel = hotkey.into();

    let updated_hotkey = hotkey::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    Ok(updated_hotkey)
}
