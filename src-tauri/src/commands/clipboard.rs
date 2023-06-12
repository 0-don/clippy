use entity::clipboard::{self, Model};
use sea_orm::{DbErr, EntityTrait};

use crate::connection;

#[tauri::command]
pub async fn infinite_scroll_clipboards(cursor: Option<String>) -> Result<Vec<Model>, ()> {
    let clipboards = get_clipboards(cursor).await;

    Ok(clipboards.unwrap())
}

async fn get_clipboards(_cursor: Option<String>) -> Result<Vec<Model>, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::Entity::find().all(&db).await?;
    Ok(model)
}
