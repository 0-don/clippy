use crate::{utils::clipboard_manager::ClipboardHelper, printlog};
use clipboard_master::{CallbackResult, ClipboardHandler};
use std::io::Error;

pub struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        printlog!("*********Clipboard changed***********");
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(async { ClipboardHelper::upsert_clipboard().await })
        });

        printlog!("*********Clipboard updated**********");

        // first copy doesnt work, so we do it twice
        // tauri::async_runtime::spawn(async { upsert_clipboard().await });

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: Error) -> CallbackResult {
        println!("Error: {}", error);
        CallbackResult::Next
    }
}
