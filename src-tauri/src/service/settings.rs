use entity::settings::{self, ActiveModel, Model};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use crate::connection;

pub async fn get_settings_db() -> Result<Model, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let settings = settings::Entity::find_by_id(1).one(&db).await?;

    Ok(settings.unwrap())
}

pub async fn update_settings_db(settings: Model) -> Result<Model, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let active_model = ActiveModel::from(settings);

    // println!("active_model: {:?}", active_model);

    let updated_settings = settings::Entity::update(active_model).exec(&db).await?;

    // println!("updated_settings: {:?}", updated_settings);

    // active_model.try_into_model()
    Ok(updated_settings)
}
