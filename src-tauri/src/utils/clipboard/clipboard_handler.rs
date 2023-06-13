use clipboard_master::{CallbackResult, ClipboardHandler};
use std::io::Error;

use super::clipboard_helper::upsert_clipboard;

pub struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async { upsert_clipboard().await })
        });

        // tauri::async_runtime::spawn(async {
        //     println!("Clipboard changed spawn thread");
        //     upsert_clipboard().await
        // });

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: Error) -> CallbackResult {
        println!("Error: {}", error);
        CallbackResult::Next
    }
}
