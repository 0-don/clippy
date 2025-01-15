use crate::prelude::*;
use entity::hotkey::{self, ActiveModel, Model};
use sea_orm::{ActiveModelTrait, EntityTrait};
use tao::connection::db;

pub async fn get_all_hotkeys_db() -> Result<Vec<Model>, DbErr> {
    let db: DatabaseConnection = db().await?;

    let hotkeys = hotkey::Entity::find().all(&db).await?;

    Ok(hotkeys)
}

pub async fn update_hotkey_db(hotkey: Model) -> Result<Model, DbErr> {
    let db: DatabaseConnection = db().await?;

    let active_model: ActiveModel = hotkey.into();

    let updated_hotkey = hotkey::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    Ok(updated_hotkey)
}
