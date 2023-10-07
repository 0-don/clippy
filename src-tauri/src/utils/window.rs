use core::time::Duration;
use std::thread;

use tauri::{Manager, WindowEvent};

use crate::utils::setup::TIMER;

use super::setup::APP;

pub fn window_event_listener() {
    let window = APP.get().unwrap().get_window("main").unwrap();

    tauri::async_runtime::spawn(async move {
        println!("window_event_listener");
        window.on_window_event(|event| match event {
            WindowEvent::Focused(true) => {
                let window = APP.get().unwrap().get_window("main").unwrap();
                println!("Window focused");

                // // Equivalent to setGlobalHotkeyEvent(true);
                window.emit("set-global-hotkey-event", true).unwrap();

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
                });

                // // Store the new timer
                {
                    let mut timer_guard = TIMER.get().unwrap().lock().unwrap();
                    *timer_guard = Some(new_timer);
                }
            }
            WindowEvent::Focused(false) => {
                println!("Window unfocused");
                let mut timer_guard = TIMER.get().unwrap().lock().unwrap();
                if let Some(existing_timer) = timer_guard.take() {
                    existing_timer.join().unwrap(); // Ensure the timer thread is stopped
                }
                // // Handle other logic
            }
            _ => {
                println!("Window event: {:?}", event)
            }
        });
    });
}
