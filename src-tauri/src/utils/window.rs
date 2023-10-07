use core::time::Duration;
use std::thread;

use tauri::{Manager, WindowEvent};

use crate::{
    service::window::toggle_main_window,
    utils::{
        hotkey::hotkey_manager::{register_hotkeys, unregister_hotkeys},
        setup::TIMER,
    },
};

use super::setup::APP;

pub fn window_event_listener() {
    let window = APP.get().unwrap().get_window("main").unwrap();

    tauri::async_runtime::spawn(async move {
        println!("window_event_listener");
        window.on_window_event(|event| match event {
            WindowEvent::Focused(true) => {
                let window = APP.get().unwrap().get_window("main").unwrap();
                register_hotkeys(true);
                println!("Window focused");

                // // Equivalent to setGlobalHotkeyEvent(true);
                window.emit("set_global_hotkey_event", true).unwrap();

                // // Clear the existing timer if there is one
                {
                    let mut timer_guard = TIMER.get().unwrap().lock().unwrap();
                    if let Some(existing_timer) = timer_guard.take() {
                        existing_timer.join().unwrap();
                    }
                }

                // // Set a new timer
                let new_timer = thread::spawn(move || {
                    thread::sleep(Duration::from_secs(5));
                    // Logic for after 5 seconds
                    window.emit("set_global_hotkey_event", false).unwrap();
                    unregister_hotkeys(false);
                });

                // // Store the new timer
                {
                    let mut timer_guard = TIMER.get().unwrap().lock().unwrap();
                    *timer_guard = Some(new_timer);
                }
            }
            WindowEvent::Focused(false) => {
                let window = APP.get().unwrap().get_window("main").unwrap();
                println!("Window unfocused");
                let mut timer_guard = TIMER.get().unwrap().lock().unwrap();
                if let Some(existing_timer) = timer_guard.take() {
                    existing_timer.join().unwrap(); // Ensure the timer thread is stopped
                }
                window.emit("set_global_hotkey_event", false).unwrap();
                unregister_hotkeys(false);
                toggle_main_window(Some(false));
                // // Handle other logic
            }
            _ => {}
        });
    });
}
