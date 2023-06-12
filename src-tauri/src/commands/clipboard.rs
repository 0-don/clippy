use entity::clipboard;
use sea_orm::EntityTrait;

use crate::connection;

#[tauri::command]
pub async fn infinite_scroll_clipboards(cursor: Option<String>) -> Result<String, ()> {
    println!("{:?}", cursor);

    let db = connection::establish_connection().await.unwrap();

    // let _clip =
    // let model = clipboard::Entity::find()
    Ok("clip.unwrap()".to_string())
}
