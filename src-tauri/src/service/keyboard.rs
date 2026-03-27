use crate::service::clipboard::get_last_clipboard_db;
use common::types::enums::ClipboardType;
use common::types::orm_query::FullClipboardDto;
use enigo::{Enigo, Keyboard, Settings};
use std::time::Duration;

pub async fn type_last_clipboard() {
    let clipboard = get_last_clipboard_db().await;

    if let Ok(clipboard_data) = clipboard {
        if let Some(content) = get_clipboard_content(&clipboard_data) {
            if content.len() < 500 {
                std::thread::sleep(Duration::from_millis(300));
                match Enigo::new(&Settings::default()) {
                    Ok(mut enigo) => {
                        let _ = enigo.text(&content);
                    }
                    Err(_) => {}
                }
            }
        }
    }
}

fn get_clipboard_content(clipboard_data: &FullClipboardDto) -> Option<String> {
    let types = ClipboardType::from_json_value(&clipboard_data.clipboard.types)?;

    types.iter().find_map(|clipboard_type| {
        match clipboard_type {
            ClipboardType::Text => clipboard_data.text.as_ref().map(|model| model.data.clone()),
            ClipboardType::Html => clipboard_data
                .text
                .as_ref()
                .map(|model| model.data.clone())
                .or_else(|| clipboard_data.html.as_ref().map(|model| model.data.clone())),
            ClipboardType::Rtf => clipboard_data
                .text
                .as_ref()
                .map(|model| model.data.clone())
                .or_else(|| clipboard_data.rtf.as_ref().map(|model| model.data.clone())),
            // Skip Image and File types as they don't have string content
            _ => None,
        }
    })
}
