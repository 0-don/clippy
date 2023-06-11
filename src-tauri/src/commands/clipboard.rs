use entity::clipboard::{self};

#[tauri::command]
pub async fn insert_clipboard(clipboard: clipboard::Model) -> Result<String, String> {
    println!("{:?}", clipboard);

    Ok("clip.unwrap()".to_string())
}
