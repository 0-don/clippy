use entity::clipboard::{self, Model};
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect, QueryTrait};

use crate::connection;

#[tauri::command]
pub async fn infinite_scroll_clipboards(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
) -> Result<Vec<Model>, ()> {
    let clipboards = get_clipboards(cursor, search, star).await;

    Ok(clipboards.unwrap())
}

async fn get_clipboards(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
) -> Result<Vec<Model>, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::Entity::find()
        .wapply_if(star, |query, starred| {
            query.filter(clipboard::Column::Star.eq(starred))
        })
        .wapply_if(search, |query, content| {
            query.filter(clipboard::Column::Content.contains(&content))
        })
        .offset(cursor)
        .limit(11)
        .all(&db)
        .await?;

    Ok(model)
}
