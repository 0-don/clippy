use entity::settings::{self, ActiveModel, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};

use crate::connection;

pub async fn get_settings_db() -> Result<Model, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let settings = settings::Entity::find_by_id(1).one(&db).await?;

    Ok(settings.unwrap())
}

pub async fn update_settings_db(settings: Model) -> Result<Model, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let active_model: ActiveModel = settings.into();

    let updated_settings = settings::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    db.close().await?;

    Ok(updated_settings)
}
