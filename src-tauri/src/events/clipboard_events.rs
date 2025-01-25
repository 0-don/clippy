use crate::{tao::global::get_app, utils::clipboard_manager::ClipboardManagerExt};
use common::types::orm_query::FullClipboardDbo;
use tauri::{Listener, Manager};
use tauri_plugin_clipboard::Clipboard;

pub fn init_clipboard_listener() {
    let clipboard = get_app().state::<Clipboard>();

    clipboard
        .start_monitor(get_app().clone())
        .expect("Failed to start clipboard monitor");

    // Use runtime::Event for Tauri v2
    let _listener = get_app().listen(
        "plugin:clipboard://clipboard-monitor/update",
        move |_event| {
            tauri::async_runtime::spawn(async {
                FullClipboardDbo::upsert_clipboard().await;
            });
        },
    );
}
