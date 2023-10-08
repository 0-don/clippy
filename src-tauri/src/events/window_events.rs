use core::time::Duration;
use tauri::WindowEvent;
use tokio::sync::oneshot;

use crate::utils::{
    hotkey_manager::{register_hotkeys, unregister_hotkeys},
    tauri::config::{MAIN_WINDOW, WINDOW_STOP_TX},
};

pub fn window_event_listener() {
    tauri::async_runtime::spawn(async move {
        let window = MAIN_WINDOW.get().unwrap();

        window
            .lock()
            .unwrap()
            .on_window_event(move |event| match event {
                WindowEvent::Focused(true) => {
                    let (tx, rx) = oneshot::channel();

                    register_hotkeys(true);
                    window
                        .lock()
                        .unwrap()
                        .emit("set_global_hotkey_event", true)
                        .unwrap();

                    tauri::async_runtime::spawn(async {
                        println!("WindowEvent::Focused(true) triggered");
                        let result = tokio::time::timeout(Duration::from_secs(5), rx).await;
                        match result {
                            Ok(_) => return, // If we're signaled, exit early
                            Err(_) => {
                                // Acquire the lock only when you need it
                                unregister_hotkeys(false);
                                window
                                    .lock()
                                    .unwrap()
                                    .emit("set_global_hotkey_event", false)
                                    .unwrap();
                            }
                        }
                    });

                    // Store the sender in the WINDOW_STOP_TX global variable
                    *WINDOW_STOP_TX.get().unwrap().lock().unwrap() = Some(tx);
                }
                WindowEvent::Focused(false) => {
                    // std::thread::sleep(Duration::from_millis(200));
                    println!("WindowEvent::Focused(false) triggered");

                    // Use the sender to signal the timer thread to exit early
                    if let Some(tx) = WINDOW_STOP_TX.get().unwrap().lock().unwrap().take() {
                        println!("STOP TIMEOUT");
                        let _ = tx.send(());
                    }

                    // toggle_main_window(Some(false));
                    unregister_hotkeys(false);

                    window.lock().unwrap().hide().unwrap();

                    window
                        .lock()
                        .unwrap()
                        .emit("set_global_hotkey_event", false)
                        .unwrap();
                }
                _ => {}
            });
    });
}
