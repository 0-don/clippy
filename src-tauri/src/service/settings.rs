use super::global::get_app;
use crate::connection;
use crate::prelude::*;
use crate::{
    commands::settings::get_settings,
    service::hotkey::with_hotkeys,
};
use common::types::types::Config;
use common::types::types::DataPath;
use entity::settings::{self, ActiveModel, Model};
use sea_orm::{ActiveModelTrait, EntityTrait};
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

pub async fn get_settings_db() -> Result<Model, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let settings = settings::Entity::find_by_id(1).one(&db).await?;

    Ok(settings.expect("Settings not found"))
}

pub async fn update_settings_db(settings: Model) -> Result<Model, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let active_model: ActiveModel = settings.into();

    let updated_settings = settings::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    Ok(updated_settings)
}

pub async fn update_settings_synchronize(sync: bool) -> Result<(), DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let settings = settings::Entity::find_by_id(1).one(&db).await?;

    let mut settings = settings.expect("Settings not found");

    settings.synchronize = sync;

    let active_model: ActiveModel = settings.into();

    let _ = settings::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    Ok(())
}

pub fn get_data_path() -> DataPath {
    let config_path = get_app()
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .to_string_lossy()
        .to_string();

    fs::create_dir_all(&config_path).expect("Failed to create config directory");

    // let config_file = Path::new(&config_dir).join("config.json");
    let config_file_path = [&config_path, "config.json"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string();

    let db_file_path = [&config_path, "clippy.sqlite"]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string();

    DataPath {
        config_path,
        db_file_path,
        config_file_path,
    }
}

pub fn get_config() -> (Config, DataPath) {
    let data_path = get_data_path();

    let json = std::fs::read_to_string(&data_path.config_file_path).expect("Failed to read file");

    let config: Config = serde_json::from_str(&json).expect("Failed to parse JSON");

    (config, data_path)
}

pub async fn sync_clipboard_history_enable() {
    // get local config from app data
    let (mut config, data_path) = get_config();

    // Use blocking_pick_folder for synchronous folder selection
    if let Some(dir) = get_app().dialog().file().blocking_pick_folder() {
        // Convert path to string
        let dir = dir.to_string();
        let dir_file = format!("{}/clippy.sqlite", &dir);

        println!("selected dir: {}", dir);

        // check if backup file exists
        if !Path::new(&dir_file).exists() {
            // copy current database to backup location
            fs::copy(&config.db, &dir_file).expect("Failed to copy database");
        }

        // overwrite config database location
        config.db = dir_file;

        // overwrite config file
        let _ = fs::write(
            &data_path.config_file_path,
            serde_json::to_string(&config).expect("Failed to serialize config"),
        );

        // Now we can await this since we're in an async function
        update_settings_synchronize(true)
            .await
            .expect("Failed to update settings");
    }
}

pub async fn sync_clipboard_history_disable() {
    let (mut config, data_path) = get_config();
    // copy backup file to default database location
    fs::copy(&config.db, &data_path.db_file_path).expect("Failed to copy database");

    // overwrite config database default location
    config.db = data_path.db_file_path;

    // overwrite config file
    fs::write(
        &data_path.config_file_path,
        serde_json::to_string(&config).expect("Failed to serialize config"),
    )
    .expect("Failed to serialize config");

    update_settings_synchronize(false)
        .await
        .expect("Failed to update settings");
}

pub async fn sync_clipboard_history_toggle() {
    let settings = get_settings().await.expect("Failed to get settings");

    with_hotkeys(false, async move {
        if settings.synchronize {
            sync_clipboard_history_disable().await;
        } else {
            sync_clipboard_history_enable().await;
        }
    })
    .await;
}
