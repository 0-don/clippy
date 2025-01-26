use crate::{
    prelude::*,
    tao::{
        connection::db,
        global::{get_app, get_main_window},
    },
    utils::hotkey_manager::register_hotkeys,
};
use common::types::enums::ListenEvent;
use entity::hotkey::{self, ActiveModel, Model};
use sea_orm::{ActiveModelTrait, EntityTrait};
use tauri::{Emitter, EventTarget};

use super::{encrypt::is_key_set, settings::get_global_settings};

pub async fn get_all_hotkeys_db() -> Result<Vec<Model>, DbErr> {
    let db: DatabaseConnection = db().await?;

    let hotkeys = hotkey::Entity::find().all(&db).await?;

    Ok(hotkeys)
}

pub async fn update_hotkey_db(hotkey: Model) -> Result<Model, DbErr> {
    let db: DatabaseConnection = db().await?;

    let active_model: ActiveModel = hotkey.into();

    let updated_hotkey = hotkey::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    Ok(updated_hotkey)
}

pub fn init_hotkey_window() {
    get_app()
        .emit_to(
            EventTarget::any(),
            ListenEvent::InitHotkeys.to_string().as_str(),
            (),
        )
        .expect("Failed to emit download progress event");
}

pub fn init_hotkey_event() {
    // If encryption is enabled and key is not set, do not register hotkeys
    if !is_key_set() && get_global_settings().encryption {
        return;
    }

    register_hotkeys(true);
    get_main_window()
        .emit(
            ListenEvent::EnableGlobalHotkeyEvent.to_string().as_str(),
            true,
        )
        .expect("Failed to emit set global hotkey event");
}
