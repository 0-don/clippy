use crate::{
    printlog,
    utils::{
        hotkey_manager::unregister_hotkeys,
        tauri::config::{HOTKEY_RUNNING, MAIN_WINDOW, WINDOW_STOP_TX},
    },
};
use core::time::Duration;
use tauri::WindowEvent;
use tokio::sync::oneshot;

pub fn window_event_listener() {
    tauri::async_runtime::spawn(async move {
        let window = MAIN_WINDOW.get().unwrap();

        window
            .lock()
            .unwrap()
            .on_window_event(move |event| match event {
                WindowEvent::Focused(true) => {
                    if !MAIN_WINDOW
                        .get()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .is_visible()
                        .unwrap()
                    {
                        return;
                    }

                    printlog!("window focus");

                    let (tx, rx) = oneshot::channel();
                    tauri::async_runtime::spawn(async move {
                        let result = tokio::time::timeout(Duration::from_secs(5), rx).await;
                        match result {
                            Ok(_) => return, // If we're signaled, exit early
                            Err(_) => {
                                // Acquire the lock only when you need it

                                unregister_hotkeys(false);

                                MAIN_WINDOW
                                    .get()
                                    .unwrap()
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
                    if !MAIN_WINDOW
                        .get()
                        .unwrap()
                        .lock()
                        .unwrap()
                        .is_visible()
                        .unwrap()
                    {
                        return;
                    }
                    printlog!("window lost focus");
                    if cfg!(target_os = "linux") {
                        std::thread::sleep(Duration::from_millis(100));
                    }
                    if *HOTKEY_RUNNING.get().unwrap().lock().unwrap() {
                        *HOTKEY_RUNNING.get().unwrap().lock().unwrap() = false;
                        return;
                    }
                    MAIN_WINDOW.get().unwrap().lock().unwrap().hide().unwrap();
                }
                _ => {}
            });
    });
}
