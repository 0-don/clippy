use super::clipboard::get_last_clipboard_db;
use super::global::get_app;
use crate::connection;
use crate::prelude::*;
use crate::service::hotkey::with_hotkeys;
use crate::service::window::get_monitor_scale_factor;
use common::constants::CONFIG_NAME;
use common::constants::DB_NAME;
use common::language::get_system_language;
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
    let db: DatabaseConnection = connection::db().await?;

    let settings = settings::Entity::find_by_id(1).one(&db).await?;

    Ok(settings.expect("Settings not found"))
}

pub async fn update_settings_db(settings: Model) -> Result<Model, DbErr> {
    let db: DatabaseConnection = connection::db().await?;

    let active_model: ActiveModel = settings.into();

    let updated_settings = settings::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    Ok(updated_settings)
}

pub async fn update_settings_synchronize(sync: bool) -> Result<(), DbErr> {
    let db: DatabaseConnection = connection::db().await?;

    let mut settings = get_settings_db().await?;

    settings.synchronize = sync;

    let active_model: ActiveModel = settings.into();

    let _ = settings::Entity::update(active_model.reset_all())
        .exec(&db)
        .await?;

    Ok(())
}

pub fn init_settings() {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            let last_clipboard = get_last_clipboard_db().await;
            if last_clipboard.is_ok() {
                return;
            }

            let mut settings = get_settings_db().await.expect("Failed to get settings");

            settings.display_scale = get_monitor_scale_factor();
            settings.language = get_system_language().to_string();

            let _ = update_settings_db(settings)
                .await
                .expect("Failed to update settings");
        })
    });
}

pub fn get_data_path() -> DataPath {
    let config_path = if cfg!(debug_assertions) {
        // Get absolute project root directory
        let current_dir = std::env::current_dir()
            .expect("Failed to get current directory")
            .parent()
            .expect("Failed to get parent directory")
            .to_path_buf();

        current_dir.to_string_lossy().to_string()
    } else {
        // Use app data dir in production
        get_app()
            .path()
            .app_data_dir()
            .expect("Failed to get app data dir")
            .to_string_lossy()
            .to_string()
    };

    fs::create_dir_all(&config_path).expect("Failed to create config directory");

    let config_file_path = [&config_path, CONFIG_NAME]
        .iter()
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_string();

    let db_file_path = [&config_path, DB_NAME]
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

        // check if backup file exists
        if !Path::new(&dir_file).exists() {
            // copy current database to backup location
            printlog!(
                "copying database to backup location {} {}",
                &config.db,
                &dir_file
            );
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
    let settings = get_settings_db().await.expect("Failed to get settings");

    printlog!("synchronize: {}", settings.synchronize);
    with_hotkeys(false, async move {
        if settings.synchronize {
            sync_clipboard_history_disable().await;
        } else {
            sync_clipboard_history_enable().await;
        }
    })
    .await;
}
