use entity::clipboard::{self, ActiveModel, Model};
use sea_orm::{ActiveModelTrait, Set};
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

    let model = ActiveModel {
        r#type: Set(clipboard.r#type),
        content: Set(clipboard.content),

        star: Set(Some(false)),

        blob: Set(clipboard.blob),
        height: Set(clipboard.height),
        width: Set(clipboard.width),
        size: Set(clipboard.size),
        ..Default::default()
    };

    let clip_db: clipboard::Model = model.insert(&db).await?;

    Ok(clip_db)
}
