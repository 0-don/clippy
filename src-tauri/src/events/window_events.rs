use crate::{
    printlog,
    service::global::{get_hotkey_running, get_main_window, get_window_stop_tx},
    utils::hotkey_manager::unregister_hotkeys,
};
use core::time::Duration;
use tauri::WindowEvent;
use tokio::sync::oneshot;

pub fn window_event_listener() {
    get_main_window().on_window_event(move |event| match event {
        WindowEvent::Focused(true) => {
            if !get_main_window().is_visible().unwrap_or(false) {
                return;
            }
            // printlog!("window focus");

            let (tx, rx) = oneshot::channel();
            tauri::async_runtime::spawn(async move {
                let result = tokio::time::timeout(Duration::from_secs(5), rx).await;
                match result {
                    Ok(_) => return, // If we're signaled, exit early
                    Err(_) => {
                        // Acquire the lock only when you need it
                        unregister_hotkeys(false);
                        get_main_window()
                            .emit("set_global_hotkey_event", false)
                            .unwrap();
                        *get_hotkey_running() = false;
                    }
                }
            });

            // Store the sender in the WINDOW_STOP_TX global variable
            *get_window_stop_tx() = Some(tx);
        }
        WindowEvent::Focused(false) => {
            if !get_main_window().is_visible().unwrap_or(false) {
                return;
            }
            tauri::async_runtime::spawn(async {
                if cfg!(target_os = "linux") {
                    std::thread::sleep(Duration::from_millis(100));
                }

                printlog!("window lost focus");

                if *get_hotkey_running() {
                    return *get_hotkey_running() = false;
                }

                if let Some(tx) = get_window_stop_tx().take() {
                    let _ = tx.send(());
                }

                if !cfg!(debug_assertions) {
                    get_main_window().hide().unwrap();
                }

                unregister_hotkeys(false);
            });
        }
        _ => {}
    });
}
