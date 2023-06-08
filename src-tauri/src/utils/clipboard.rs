extern crate alloc;
use alloc::borrow::Cow;
use arboard::{Clipboard, ImageData};
use clipboard_master::{CallbackResult, ClipboardHandler};
use std::{io, process::Command};

pub struct Handler;

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        Command::new("clear").status().unwrap();
        
        let mut clipboard = Clipboard::new().unwrap();

        let text = match clipboard.get_text() {
            Ok(text) => text,
            Err(_) => "".to_string(),
        };

        let image = match clipboard.get_image() {
            Ok(image) => image,
            Err(_) => ImageData {
                width: 0,
                height: 0,
                bytes: Cow::from([0].as_ref()),
            },
        };

        println!("{}", text);
        println!("{:?} {:?}", image.height, image.width);

        let blob = image.bytes.into_owned();

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}
