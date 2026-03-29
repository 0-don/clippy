// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok();

    #[cfg(target_os = "linux")]
    {
        // See: https://github.com/spacedriveapp/spacedrive/issues/1512#issuecomment-1758550164
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

        // GNOME does not support SNI tray icons or Wayland data-control protocol.
        // Force X11 backend via XWayland so tray and clipboard work correctly.
        // Other compositors (KDE, wlroots) support both natively on Wayland.
        // See: https://github.com/tauri-apps/tauri/issues/14234
        let is_gnome = std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_default()
            .to_uppercase()
            .contains("GNOME");
        if is_gnome && std::env::var("WAYLAND_DISPLAY").is_ok() {
            std::env::set_var("GDK_BACKEND", "x11");
        }
    }

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri_app_lib::run();

    Ok(())
}
