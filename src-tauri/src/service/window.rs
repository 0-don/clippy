use tauri::Manager;

use crate::utils::setup::APP;

pub fn init_event() {
    APP.get()
        .unwrap()
        .get_window("main")
        .unwrap()
        .emit("init_listener", Some(()))
        .unwrap();
}
