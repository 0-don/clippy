use crate::connection;
use entity::settings::{self, ActiveModel, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};

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

    Ok(updated_settings)
}

pub async fn update_settings_synchronize(sync: bool) -> Result<(), DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let settings = settings::Entity::find_by_id(1).one(&db).await?;

    let mut settings = settings.unwrap();

    settings.synchronize = sync;

    let active_model: ActiveModel = settings.into();

    let _ = settings::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    Ok(())
}
