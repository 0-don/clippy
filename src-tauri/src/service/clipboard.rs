extern crate alloc;

use crate::{connection, utils::clipboard::clipboard_helper::parse_model};
use entity::clipboard::{self, Model};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
    QueryTrait, Set,
};

pub async fn upsert_db() -> Result<Model, DbErr> {
    let clipboard = parse_model();

    let db = connection::establish_connection().await;

    let clip_db: Model = clipboard.insert(&db.unwrap()).await?;

    Ok(clip_db)
}

pub async fn get_clipboards_db(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
) -> Result<Vec<Model>, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::Entity::find()
        .apply_if(star, |query, starred| {
            query.filter(clipboard::Column::Star.eq(starred))
        })
        .apply_if(search, |query, content| {
            query.filter(clipboard::Column::Content.contains(&content))
        })
        .offset(cursor)
        .limit(11)
        .order_by_desc(clipboard::Column::Id)
        .all(&db)
        .await?;

    Ok(model)
}

pub async fn star_clipboard_db(id: i32, star: bool) -> Result<Option<bool>, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::ActiveModel {
        id: Set(id),
        star: Set(Some(star)),
        ..Default::default()
    };

    let _clipboard = clipboard::Entity::update(model).exec(&db).await?;

    Ok(Some(true))
}

pub async fn delete_clipboard_db(id: i32) -> Result<Option<bool>, DbErr> {
    let db = connection::establish_connection().await?;

    clipboard::Entity::delete_by_id(id).exec(&db).await?;

    Ok(Some(true))
}
