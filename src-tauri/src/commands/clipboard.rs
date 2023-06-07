use entity::clipboard::{self, ActiveModel, Model};
use sea_orm::ActiveModelTrait;
use sea_orm::{DatabaseConnection, DbErr};

use crate::connection;

#[tauri::command]
pub async fn insert_clipboard(clipboard: clipboard::Model) -> Result<Model, String> {
    println!("{:?}", clipboard);
    let clip = insert(clipboard).await;

    Ok(clip.unwrap())
}

async fn insert(clipboard: clipboard::Model) -> Result<Model, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let clip: ActiveModel = clipboard.into();

    let clip_db: clipboard::Model = clip.insert(&db).await?;

    Ok(clip_db)
}
