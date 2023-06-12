use arboard::{Clipboard, ImageData};
use entity::clipboard::{self, ActiveModel, Model};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

use crate::connection;

pub async fn upsert(clipboard: ActiveModel) -> Result<Option<Model>, DbErr> {
    let is_same = check_if_last_same().await;
    if is_same.is_some() {
        ()
    }
    let db: DatabaseConnection = connection::establish_connection().await?;

    let clip_db: Model = clipboard.insert(&db).await?;

    Ok(Some(clip_db))
}

pub async fn get_clipboards(
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
        .order_by_desc(clipboard::Column::Id)
        .all(&db)
        .await?;

    Ok(model)
}

pub async fn delete_clipboard_db(id: i32) -> Result<Option<bool>, DbErr> {
    let db = connection::establish_connection().await?;

    clipboard::Entity::delete_by_id(id).exec(&db).await?;

    Ok(Some(true))
}

async fn check_if_last_same() -> Option<Model> {
    let (text, image) = get_os_clipboard();

    let str = text.unwrap();
    let img = image.unwrap();

    let db = connection::establish_connection().await.unwrap();

    let last_clipboard = clipboard::Entity::find()
        .order_by_desc(clipboard::Column::Id)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    let content = last_clipboard.clone().content.unwrap();
    let blob = last_clipboard.clone().blob.unwrap();

    if content == str && blob == img.bytes.to_vec() {
        return None;
    }

    Some(last_clipboard)
}

pub fn get_os_clipboard() -> (Option<String>, Option<ImageData<'static>>) {
    // Command::new("clear").status().unwrap();

    let mut clipboard = Clipboard::new().unwrap();

    let text: Option<String> = match clipboard.get_text() {
        Ok(text) => Some(text),
        Err(_) => None,
    };

    let image: Option<ImageData<'_>> = match clipboard.get_image() {
        Ok(image) => Some(image),
        Err(_) => None,
    };

    (text, image)
}
