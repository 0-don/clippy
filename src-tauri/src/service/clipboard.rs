extern crate alloc;

use crate::connection;
use entity::clipboard::{self, ActiveModel, Model};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
    QueryTrait, Set,
};

pub async fn insert_clipboard_db(clipboard: ActiveModel) -> Result<Model, DbErr> {
    let db = connection::establish_connection().await;

    let mut clip_db: Model = clipboard.insert(&db.unwrap()).await?;

    if clip_db.content.is_some() && clip_db.content.as_ref().unwrap().len() > 100 {
        clip_db.content = clip_db.content.unwrap()[..100].to_string().into();
    }

    Ok(clip_db)
}

pub async fn get_clipboard_db(id: i32) -> Result<Model, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::Entity::find_by_id(id).one(&db).await?;

    Ok(model.unwrap())
}

pub async fn get_clipboards_db(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
    img: Option<bool>,
) -> Result<Vec<Model>, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::Entity::find()
        .apply_if(star, |query, starred| {
            query.filter(clipboard::Column::Star.eq(starred))
        })
        .apply_if(search, |query, content| {
            query.filter(clipboard::Column::Content.contains(&content))
        })
        .apply_if(img, |query, _| {
            query.filter(clipboard::Column::Type.eq("image"))
        })
        .offset(cursor)
        .limit(50)
        .order_by_desc(clipboard::Column::Id)
        .all(&db)
        .await?;

    let parsed_model: Vec<Model> = model
        .into_iter()
        .map(|mut m| {
            if m.content.is_some() && m.content.as_ref().unwrap().len() > 100 {
                m.content = m.content.unwrap()[..100].to_string().into();
            }
            m
        })
        .collect();

    Ok(parsed_model)
}

pub async fn star_clipboard_db(id: i32, star: bool) -> Result<bool, DbErr> {
    let db = connection::establish_connection().await?;

    let model = clipboard::ActiveModel {
        id: Set(id),
        star: Set(Some(star)),
        ..Default::default()
    };

    println!("model: {:?}", model);

    let _clipboard = clipboard::Entity::update(model).exec(&db).await?;

    Ok(true)
}

pub async fn delete_clipboard_db(id: i32) -> Result<bool, DbErr> {
    let db = connection::establish_connection().await?;

    clipboard::Entity::delete_by_id(id).exec(&db).await?;

    Ok(true)
}
