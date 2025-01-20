use crate::service::window::toggle_main_window;
use tao::global::get_app;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

pub fn init_system_tray() -> Result<(), Box<dyn std::error::Error>> {
    // Create menu items
    let quit = MenuItem::with_id(get_app(), "quit", "Quit", true, None::<&str>)?;
    let open = MenuItem::with_id(get_app(), "open", "Open", true, None::<&str>)?;

    // Create the menu
    let menu = Menu::with_items(get_app(), &[&open, &quit])?;

    // Build and return the tray
    TrayIconBuilder::new()
        .icon(
            get_app()
                .default_window_icon()
                .expect("failed to get default icon")
                .to_owned(),
        )
        .menu(&menu)
        .on_menu_event(|app, event| {
            // Use event.id.0 to get the string value
            match event.id.0.as_str() {
                "open" => toggle_main_window(),
                "quit" => app.exit(0),
                id => println!("Unhandled menu item: {:?}", id),
            }
        })
        .on_tray_icon_event(|_tray, event| {
            use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};

            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => toggle_main_window(),
                TrayIconEvent::DoubleClick {
                    button: MouseButton::Left,
                    ..
                } => toggle_main_window(),
                _ => (),
            }
        })
        .build(get_app())?;

    Ok(())
}
