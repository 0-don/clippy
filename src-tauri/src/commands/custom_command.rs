use crate::{
    service::{custom_command::run_command, settings::update_settings_commands},
    utils::hotkey_manager::reload_global_hotkeys,
};
use common::types::types::{CommandError, CustomCommand};

#[tauri::command]
pub async fn change_settings_commands(
    commands: Vec<CustomCommand>,
) -> Result<Vec<CustomCommand>, CommandError> {
    let commands = update_settings_commands(commands).await?;
    reload_global_hotkeys().await;
    Ok(commands)
}

#[tauri::command]
pub async fn run_custom_command(command: CustomCommand) -> Result<String, CommandError> {
    run_command(&command).await.map_err(CommandError::Error)
}
