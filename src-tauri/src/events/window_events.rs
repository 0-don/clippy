use crate::tao::global::{get_hotkey_running, get_window_stop_tx};
use crate::utils::hotkey_manager::unregister_hotkeys;
use crate::{prelude::*, tao::global::get_main_window};
use common::types::enums::ListenEvent;
use tauri::{Emitter, WindowEvent};
use tokio::sync::oneshot;

pub fn setup_window_event_listener() {
    get_main_window().on_window_event(|event| {
        if !get_main_window().is_visible().unwrap_or(false) {
            return;
        }

        match event {
            WindowEvent::Focused(true) => {
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
                                ListenEvent::EnableGlobalHotkeyEvent.to_string().as_str(),
                                false,
                            )
                            .expect("failed to emit event");
                        *get_hotkey_running() = false;
                    }
                });
            }
            WindowEvent::Focused(false) => {
                tauri::async_runtime::spawn(async {
                    if cfg!(target_os = "linux") {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }

                    // STEP 1: Always unregister the hotkeys immediately.
                    // This releases the keyboard grab and fixes the lock-up.
                    unregister_hotkeys(false);

                    // STEP 2: Complete the oneshot channel from the Focused(true) handler.
                    if let Some(tx) = get_window_stop_tx().take() {
                        tx.send(()).unwrap_or(());
                    }

                    // STEP 3: Use the flag ONLY to decide whether to hide the window.
                    // If a hotkey was just used, we don't hide the window, but we still reset the flag.
                    if *get_hotkey_running() {
                        *get_hotkey_running() = false;
                        return; // Exit without hiding
                    }

                    // // STEP 4: If no hotkey was just used, hide the window.
                    if !cfg!(debug_assertions) {
                        get_main_window().hide().expect("failed to hide window");
                    }
                });
            }
            _ => {}
        }
    });
}
