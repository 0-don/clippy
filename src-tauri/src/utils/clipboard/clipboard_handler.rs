use clipboard_master::{CallbackResult, ClipboardHandler};
use std::io::Error;

use super::clipboard_helper::upsert_clipboard;

pub struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        // let res = tokio::task::block_in_place(|| {
        //     tokio::runtime::Handle::current().block_on(async move {
        //         let model = parse_model();
        //         insert(model).await
        //     })
        // });

        println!("Clipboard changed");

        tauri::async_runtime::spawn(async { upsert_clipboard().await });

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: Error) -> CallbackResult {
        println!("Error: {}", error);
        CallbackResult::Next
    }
}
