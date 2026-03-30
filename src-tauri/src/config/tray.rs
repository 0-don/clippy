use crate::{
    service::{settings::get_global_settings, window::toggle_main_window},
    tao::global::get_app,
};
use common::types::enums::Language;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconId},
    Manager,
};

const TRAY_ID: &str = "clippy_tray";

fn build_tray_menu() -> Result<Menu<tauri::Wry>, Box<dyn std::error::Error>> {
    let settings = get_global_settings();
    let lang = Language::from_iso_code(&settings.language);
    let labels = lang.tray_labels();

    let quit = MenuItem::with_id(get_app(), "quit", labels.quit, true, None::<&str>)?;
    let open = MenuItem::with_id(get_app(), "open", labels.open, true, None::<&str>)?;

    Ok(Menu::with_items(get_app(), &[&open, &quit])?)
}

pub fn setup_system_tray() -> Result<(), Box<dyn std::error::Error>> {
    let menu = build_tray_menu()?;

    let version = get_app().package_info().version.to_string();

    TrayIconBuilder::with_id(TRAY_ID)
        .icon(
            get_app()
                .default_window_icon()
                .expect("failed to get default icon")
                .to_owned(),
        )
        .temp_dir_path(get_app().path().app_cache_dir().unwrap_or_default())
        .tooltip(format!("clippy {}", version))
        .menu(&menu)
        .on_menu_event(|app, event| {
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

pub fn refresh_tray_menu() {
    let menu = match build_tray_menu() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to build tray menu: {}", e);
            return;
        }
    };

    if let Some(tray) = get_app().tray_by_id(&TrayIconId::new(TRAY_ID)) {
        tray.set_menu(Some(menu)).ok();
    }
}
