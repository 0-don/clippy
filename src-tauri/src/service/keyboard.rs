use crate::{
    service::clipboard::get_last_clipboard_db, types::orm_query::ClipboardWithRelations,
    utils::tauri::config::APP,
};
use enigo::{Enigo, Keyboard, Settings};
use migration::ClipboardType;
use sea_orm::Iden;
use std::{process::Command, time::Duration};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_dialog::{MessageDialogButtons, MessageDialogKind};

pub async fn type_last_clipboard() {
    let clipboard = get_last_clipboard_db().await;

    if let Ok(clipboard_data) = clipboard {
        if let Some(content) = get_clipboard_content(&clipboard_data) {
            if content.len() < 32 {
                let mut enigo = Enigo::new(&Settings::default()).unwrap();
                let _ = enigo.text(&content);
            }
        }
    }
}

pub async fn type_last_clipboard_linux() -> Result<(), Box<dyn std::error::Error>> {
    if !is_tool_installed("xdotool") {
        APP.get()
            .unwrap()
            .dialog()
            .message("xdotool is not installed. Please install it to continue.")
            .title("Missing Dependency")
            .kind(MessageDialogKind::Error)
            .buttons(MessageDialogButtons::Ok)
            .blocking_show();
        return Ok(());
    }

    let clipboard = get_last_clipboard_db().await;

    if let Ok(clipboard_data) = clipboard {
        if let Some(content) = get_clipboard_content(&clipboard_data) {
            // TODO: add the limit to db
            if content.len() < 500 {
                std::thread::sleep(Duration::from_millis(300));
                Command::new("xdotool")
                    .args(&["type", "--clearmodifiers", "--", &content])
                    .output()?;
            }
        }
    }

    Ok(())
}

pub fn is_tool_installed(tool: &str) -> bool {
    Command::new(tool)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn get_clipboard_content(clipboard_data: &ClipboardWithRelations) -> Option<String> {
    match clipboard_data.clipboard.r#type.as_str() {
        type_str if type_str == ClipboardType::Text.to_string() => {
            clipboard_data.text.as_ref().map(|model| model.data.clone())
        }
        type_str if type_str == ClipboardType::Html.to_string() => clipboard_data
            .text
            .as_ref()
            .map(|model| model.data.clone())
            .or_else(|| clipboard_data.html.as_ref().map(|model| model.data.clone())),
        type_str if type_str == ClipboardType::Rtf.to_string() => clipboard_data
            .text
            .as_ref()
            .map(|model| model.data.clone())
            .or_else(|| clipboard_data.rtf.as_ref().map(|model| model.data.clone())),
        _ => None,
    }
}
