extern crate alloc;
use crate::utils::setup::APP;

use clipboard_master::{CallbackResult, ClipboardHandler};
use std::io;
use tauri::Manager;

use super::clipboard_helper::check_clipboard;

pub struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        // let res = tokio::task::block_in_place(|| {
        //     tokio::runtime::Handle::current().block_on(async move {
        //         let model = parse_model();
        //         insert(model).await
        //     })
        // });

        let _ = tauri::async_runtime::spawn(async move {
            let model = check_clipboard().await;

            APP.get()
                .unwrap()
                .get_window("main")
                .unwrap()
                .emit("clipboard_listener", model)
                .unwrap();
        });

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        println!("Error: {}", error);
        CallbackResult::Next
    }
}
