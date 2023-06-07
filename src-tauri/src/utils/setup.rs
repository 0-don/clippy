use once_cell::sync::OnceCell;

pub static MAIN_WINDOW_X: i32 = 600;
pub static MAIN_WINDOW_Y: i32 = 375;

pub static APP: OnceCell<tauri::AppHandle> = OnceCell::new();

pub fn setup(app: &mut tauri::App) -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    APP.set(app.handle()).expect("error initializing tauri app");
    Ok(())
}
