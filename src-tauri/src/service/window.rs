use std::sync::{Arc, Mutex};

use super::global::{get_app, get_main_window};
use crate::{
    printlog,
    service::global::get_window_stop_tx,
    types::types::WindowName,
    utils::{
        hotkey_manager::{register_hotkeys, unregister_hotkeys},
        tauri::config::{MAIN_WINDOW, MAIN_WINDOW_X, MAIN_WINDOW_Y},
    },
};
use tauri::{Emitter, LogicalSize, Manager, WebviewUrl};
use tauri::{PhysicalPosition, WebviewWindowBuilder};

pub fn init_window(app: &mut tauri::App) {
    let window: tauri::WebviewWindow = app.get_webview_window("main").unwrap();
    let _ = window.set_size(LogicalSize::new(MAIN_WINDOW_X, MAIN_WINDOW_Y));

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
        .unwrap_or_else(|_| panic!("Failed to initialize MAIN_WINDOW"));
}

pub fn toggle_main_window() {
    if get_main_window().is_visible().unwrap() {
        printlog!("hiding window");
        if let Some(tx) = get_window_stop_tx().take() {
            let _ = tx.send(());
        }

        get_main_window().hide().unwrap();
        unregister_hotkeys(false);
        get_main_window()
            .emit("set_global_hotkey_event", false)
            .unwrap();
    } else {
        // get_main_window().move_window(Position::Center).unwrap();
        position_window_near_cursor();
        get_main_window()
            .emit("change_tab", "recent_clipboards")
            .unwrap();
        get_main_window().show().unwrap();

        register_hotkeys(true);
        get_main_window()
            .emit("set_global_hotkey_event", true)
            .unwrap();

        get_app()
            .run_on_main_thread(|| get_main_window().set_focus().unwrap())
            .unwrap();

        printlog!("displaying window");
    }
}

pub fn position_window_near_cursor() {
    let window = get_main_window();

    if let Ok(cursor_position) = window.cursor_position() {
        let window_size = window.outer_size().unwrap();

        // Get current monitor or fallback to primary
        let current_monitor = window
            .available_monitors()
            .unwrap()
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
            .unwrap_or_else(|| window.primary_monitor().unwrap().unwrap());

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

        window.set_position(final_pos).unwrap();
    }
}

pub fn create_about_window() {
    let app = crate::service::global::get_app();

    // Close existing window if it exists
    if let Some(window) = app.get_webview_window("about") {
        let _ = window.close();
    }

    WebviewWindowBuilder::new(app, "about", WebviewUrl::App("pages/about.html".into()))
        .title("About")
        .inner_size(375.0, 600.0)
        .always_on_top(true)
        .build()
        .unwrap();
}

pub fn create_settings_window() {
    let app = crate::service::global::get_app();

    // Close existing window if it exists
    if let Some(window) = app.get_webview_window("settings") {
        let _ = window.close();
    }

    WebviewWindowBuilder::new(
        app,
        "settings",
        WebviewUrl::App("pages/settings.html".into()),
    )
    .title("Settings")
    .inner_size(500.0, 450.0)
    .always_on_top(true)
    .build()
    .unwrap();
}

pub fn open_window(window_name: WindowName) {
    match window_name {
        WindowName::About => create_about_window(),
        WindowName::Settings => create_settings_window(),
    }
}
