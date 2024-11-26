use crate::utils::clipboard_manager::ClipboardHelper;
use tauri::{Listener, Manager};
use tauri_plugin_clipboard::Clipboard;

pub fn init_clipboard_listener(app: &mut tauri::App) {
    let clipboard = app.handle().state::<Clipboard>();

    clipboard
        .start_monitor(app.handle().clone())
        .expect("Failed to start clipboard monitor");

    // Use runtime::Event for Tauri v2
    let _listener = app.handle().listen(
        "plugin:clipboard://clipboard-monitor/update",
        move |_event| {
            tauri::async_runtime::spawn(async {
                ClipboardHelper::upsert_clipboard().await;
            });
        },
    );
}
