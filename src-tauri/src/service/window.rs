use super::global::{get_app, get_main_window};
use crate::prelude::*;
use crate::tauri_config::config::{MAIN_WINDOW, MAIN_WINDOW_X, MAIN_WINDOW_Y, MAX_IMAGE_SIZE};
use crate::{
    printlog,
    service::global::get_window_stop_tx,
    utils::hotkey_manager::{register_hotkeys, unregister_hotkeys},
};
use common::enums::{CommandEvent, WebWindow};
use std::sync::{Arc, Mutex};
use tauri::{Emitter, LogicalSize, Manager, WebviewUrl};
use tauri::{PhysicalPosition, WebviewWindowBuilder};

pub fn init_window(app: &mut tauri::App) {
    let window: tauri::WebviewWindow = app
        .get_webview_window(WebWindow::Main.to_string().as_str())
        .expect("Failed to get window");
    window
        .set_size(LogicalSize::new(MAIN_WINDOW_X, MAIN_WINDOW_Y))
        .expect("Failed to set window size");

    #[cfg(any(windows, target_os = "macos"))]
    {
        let _ = window.set_decorations(false);
        let _ = window.set_shadow(true);
    }

    #[cfg(debug_assertions)]
    {
        window.open_devtools();
    }
    MAIN_WINDOW
        .set(Arc::new(Mutex::new(window)))
        .expect("Failed to set main window");
}

pub fn toggle_main_window() {
    if get_main_window()
        .is_visible()
        .expect("Failed to check if window is visible")
    {
        printlog!("hiding window");
        if let Some(tx) = get_window_stop_tx().take() {
            tx.send(()).expect("Failed to send stop signal");
        }

        get_main_window().hide().expect("Failed to hide window");
        unregister_hotkeys(false);
        get_main_window()
            .emit(
                CommandEvent::SetGlobalHotkeyEvent.to_string().as_str(),
                false,
            )
            .expect("Failed to emit set global hotkey event");
    } else {
        position_window_near_cursor();
        get_main_window()
            .emit(
                CommandEvent::ChangeTab.to_string().as_str(),
                "recent_clipboards",
            )
            .expect("Failed to emit change tab event");
        get_main_window().show().expect("Failed to show window");

        register_hotkeys(true);
        get_main_window()
            .emit(
                CommandEvent::SetGlobalHotkeyEvent.to_string().as_str(),
                true,
            )
            .expect("Failed to emit set global hotkey event");

        get_app()
            .run_on_main_thread(|| get_main_window().set_focus().expect("Failed to set focus"))
            .expect("Failed to run on main thread");

        printlog!("displaying window");
    }
}

pub fn position_window_near_cursor() {
    let window = get_main_window();

    if let Ok(cursor_position) = window.cursor_position() {
        let window_size = window.outer_size().expect("Failed to get window size");

        // Get current monitor or fallback to primary
        let current_monitor = window
            .available_monitors()
            .expect("Failed to get available monitors")
            .into_iter()
            .find(|monitor| {
                let pos = monitor.position();
                let size = monitor.size();
                let bounds = (
                    pos.x as f64,
                    pos.y as f64,
                    pos.x as f64 + size.width as f64,
                    pos.y as f64 + size.height as f64,
                );

                cursor_position.x >= bounds.0
                    && cursor_position.x < bounds.2
                    && cursor_position.y >= bounds.1
                    && cursor_position.y < bounds.3
            })
            .unwrap_or_else(|| {
                window
                    .primary_monitor()
                    .expect("Failed to get primary monitor")
                    .expect("Failed to get primary monitor")
            });

        let scale_factor = current_monitor.scale_factor();
        let monitor_pos = current_monitor.position();
        let monitor_size = current_monitor.size();

        // Calculate window position with offset
        let pos = PhysicalPosition::new(
            ((cursor_position.x + 10.0) * scale_factor) as i32,
            ((cursor_position.y + 10.0) * scale_factor) as i32,
        );

        // Calculate monitor bounds in physical pixels
        let monitor_bounds = (
            (monitor_pos.x as f64 * scale_factor) as i32,
            (monitor_pos.y as f64 * scale_factor) as i32,
            (monitor_pos.x as f64 * scale_factor + monitor_size.width as f64 * scale_factor) as i32,
            (monitor_pos.y as f64 * scale_factor + monitor_size.height as f64 * scale_factor)
                as i32,
        );

        // Constrain window position within monitor bounds
        let final_pos = PhysicalPosition::new(
            pos.x
                .max(monitor_bounds.0)
                .min(monitor_bounds.2 - window_size.width as i32),
            pos.y
                .max(monitor_bounds.1)
                .min(monitor_bounds.3 - window_size.height as i32),
        );

        window
            .set_position(final_pos)
            .expect("Failed to set window position");
    }
}

pub fn calculate_thumbnail_dimensions(width: u32, height: u32) -> (u32, u32) {
    let aspect_ratio = width as f64 / height as f64;
    if width > MAX_IMAGE_SIZE || height > MAX_IMAGE_SIZE {
        if width > height {
            (
                MAX_IMAGE_SIZE,
                (MAX_IMAGE_SIZE as f64 / aspect_ratio) as u32,
            )
        } else {
            (
                (MAX_IMAGE_SIZE as f64 * aspect_ratio) as u32,
                MAX_IMAGE_SIZE,
            )
        }
    } else {
        (width, height)
    }
}

pub fn create_about_window() {
    let app = crate::service::global::get_app();

    // Close existing window if it exists
    if let Some(window) = app.get_webview_window(WebWindow::About.to_string().as_str()) {
        window.close().expect("Failed to close window");
    }

    WebviewWindowBuilder::new(
        app,
        WebWindow::About.to_string().as_str(),
        WebviewUrl::App("pages/about.html".into()),
    )
    .title("About")
    .inner_size(375.0, 600.0)
    .always_on_top(true)
    .build()
    .expect("Failed to build window");
}

pub fn create_settings_window() {
    let app = crate::service::global::get_app();

    // Close existing window if it exists
    if let Some(window) = app.get_webview_window(WebWindow::Settings.to_string().as_str()) {
        window.close().expect("Failed to close window");
    }

    WebviewWindowBuilder::new(
        app,
        WebWindow::Settings.to_string().as_str(),
        WebviewUrl::App("pages/settings.html".into()),
    )
    .title("Settings")
    .inner_size(500.0, 450.0)
    .always_on_top(true)
    .build()
    .expect("Failed to build window");
}

pub fn open_window(window_name: WebWindow) {
    match window_name {
        WebWindow::About => create_about_window(),
        WebWindow::Settings => create_settings_window(),
        _ => {}
    }
}
