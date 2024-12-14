use crate::prelude::*;
use crate::{
    service::global::{get_hotkey_running, get_main_window, get_window_stop_tx},
    utils::hotkey_manager::unregister_hotkeys,
};
use common::types::enums::ListenEvent;
use tauri::{Emitter, WindowEvent};
use tokio::sync::oneshot;

pub fn window_event_listener() {
    get_main_window().on_window_event(|event| {
        if !get_main_window().is_visible().unwrap_or(false) {
            return;
        }

        match event {
            WindowEvent::Focused(true) => {
                printlog!("window focus");
                let (tx, rx) = oneshot::channel();
                *get_window_stop_tx() = Some(tx);

                tauri::async_runtime::spawn(async move {
                    if tokio::time::timeout(std::time::Duration::from_secs(5), rx)
                        .await
                        .is_err()
                    {
                        unregister_hotkeys(false);
                        get_main_window()
                            .emit(
                                ListenEvent::SetGlobalHotkeyEvent.to_string().as_str(),
                                false,
                            )
                            .expect("failed to emit event");
                        *get_hotkey_running() = false;
                    }
                });
            }
            WindowEvent::Focused(false) => {
                printlog!("window lost focus");
                tauri::async_runtime::spawn(async {
                    if cfg!(target_os = "linux") {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }

                    if *get_hotkey_running() {
                        return *get_hotkey_running() = false;
                    }

                    if let Some(tx) = get_window_stop_tx().take() {
                        tx.send(()).unwrap_or(());
                    }

                    if !cfg!(debug_assertions) {
                        get_main_window().hide().expect("failed to hide window");
                    }

                    unregister_hotkeys(false);
                });
            }
            _ => {}
        }
    });
}
