extern crate alloc;

use alloc::borrow::Cow;
use arboard::{Clipboard, ImageData};
use entity::clipboard::Model;
use tauri::Manager;

use crate::{
    service::clipboard::{
        delete_clipboard_db, get_clipboard_db, get_clipboards_db, star_clipboard_db,
    },
    utils::setup::APP,
};

#[tauri::command]
pub async fn get_clipboards(
    cursor: Option<u64>,
    search: Option<String>,
    star: Option<bool>,
    img: Option<bool>,
) -> Result<Vec<Model>, ()> {
    let clipboards = get_clipboards_db(cursor, search, star, img).await;
    Ok(clipboards.unwrap())
}

#[tauri::command]
pub async fn copy_clipboard(id: i32) -> Result<(), ()> {
    let clipboard = get_clipboard_db(id).await;

    if clipboard.is_ok() {
        let mut clip = Clipboard::new().unwrap();
        let r#type = &clipboard.as_ref().unwrap().r#type;

        if r#type == "image" {
            let clipboard_ref = clipboard.as_ref().unwrap();
            let width = clipboard_ref.width.unwrap() as usize;
            let height = clipboard_ref.height.unwrap() as usize;
            let blob = clipboard_ref.blob.as_ref().unwrap();

            let image = image::load_from_memory(blob).unwrap();

            let img_data = ImageData {
                width,
                height,
                bytes: Cow::from(image.as_bytes()),
            };

            clip.set_image(img_data).unwrap();
        } else {
            let content = clipboard.unwrap().content.unwrap();
            clip.set_text(content).unwrap();
        }

        APP.get()
            .unwrap()
            .get_window("main")
            .unwrap()
            .hide()
            .unwrap();
    }

    Ok(())
}

#[tauri::command]
pub async fn star_clipboard(id: i32, star: bool) -> Result<bool, ()> {
    let clipboard = star_clipboard_db(id, star).await;

    Ok(clipboard.unwrap())
}

#[tauri::command]
pub async fn delete_clipboard(id: i32) -> Result<bool, ()> {
    let clipboard = delete_clipboard_db(id).await;

    Ok(clipboard.unwrap())
}
