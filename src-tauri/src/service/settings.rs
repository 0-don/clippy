use super::global::get_app;
use crate::connection;
use crate::{
    commands::settings::get_settings,
    service::hotkey::with_hotkeys,
    types::types::{Config, DataPath},
};
use entity::settings::{self, ActiveModel, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
use std::{
    fs::{self},
    path::{Path, PathBuf},
};
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

pub async fn get_settings_db() -> Result<Model, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let settings = settings::Entity::find_by_id(1).one(&db).await?;

    Ok(settings.unwrap())
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

    let mut settings = settings.unwrap();

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
        .unwrap()
        .to_string_lossy()
        .to_string();

    let _ = fs::create_dir_all(&config_path);

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

    let json = std::fs::read_to_string(&data_path.config_file_path).unwrap();

    let config: Config = serde_json::from_str(&json).unwrap();

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
            let _ = fs::copy(&config.db, &dir_file);
        }

        // overwrite config database location
        config.db = dir_file;

        // overwrite config file
        let _ = fs::write(
            &data_path.config_file_path,
            serde_json::to_string(&config).unwrap(),
        );

        // Now we can await this since we're in an async function
        update_settings_synchronize(true).await.unwrap();
    }
}

pub async fn sync_clipboard_history_disable() {
    let (mut config, data_path) = get_config();
    // copy backup file to default database location
    let _ = fs::copy(&config.db, &data_path.db_file_path);

    // overwrite config database default location
    config.db = data_path.db_file_path;

    // overwrite config file
    let _ = fs::write(
        &data_path.config_file_path,
        serde_json::to_string(&config).unwrap(),
    );

    update_settings_synchronize(false).await.unwrap();
}

pub async fn sync_clipboard_history_toggle() {
    let settings = get_settings().await.unwrap();

    with_hotkeys(false, async move {
        if settings.synchronize {
            sync_clipboard_history_disable().await;
        } else {
            sync_clipboard_history_enable().await;
        }
    })
    .await;
}
