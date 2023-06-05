use entity::clipboard;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm::{DatabaseConnection, DbErr};

use crate::connection;

#[tauri::command]
pub async fn greet(name: &str) -> Result<String, String> {
    let res = insert().await;

    Ok(format!(
        "Hello, {}! You've been greeted from Rust! {}",
        res.unwrap(),
        name,
    ))
}

pub async fn insert() -> Result<String, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let post = clipboard::ActiveModel {
        r#type: Set(String::from("textx")),
        content: Set(Some(String::from("Hello, World!"))),
        ..Default::default()
    };

    let post: clipboard::Model = post.insert(&db).await?;

    Ok(format!("Post created with ID: {}", post.id))
}
