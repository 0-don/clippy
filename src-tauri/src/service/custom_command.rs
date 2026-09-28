use super::{keyboard::paste_into_active_window, settings::get_global_settings};
use crate::{prelude::*, tao::global::get_app};
use common::types::{
    enums::CommandOutput,
    types::{CustomCommand, COMMAND_EVENT_PREFIX},
};
use std::{path::Path, process::Stdio, time::Duration};
use tauri::Manager;
use tauri_plugin_clipboard::Clipboard;
use tauri_plugin_notification::NotificationExt;
use tokio::io::AsyncWriteExt;

const COMMAND_TIMEOUT: Duration = Duration::from_secs(30);

/// Windows caps one environment variable at 32767 characters; stdin always carries the full text.
const MAX_ENV_TEXT_LEN: usize = 32_000;

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub fn find_command(event: &str) -> Option<CustomCommand> {
    let id = event.strip_prefix(COMMAND_EVENT_PREFIX)?;
    CustomCommand::from_json_value(&get_global_settings().commands)
        .into_iter()
        .find(|command| command.id == id)
}

pub async fn run_command_hotkey(command: CustomCommand) {
    match run_command(&command).await {
        Ok(stdout) => apply_output(&command, &stdout).await,
        Err(error) => notify_failure(&command.name, &error),
    }
}

/// Runs the script for this OS with the clipboard text on stdin and in `CLIPPY_TEXT`.
pub async fn run_command(command: &CustomCommand) -> Result<String, String> {
    let script = command.scripts.current();
    let (program, args, extension) = resolve_interpreter(&script.interpreter);

    // A script file sidesteps quoting rules that differ per interpreter.
    let body = if extension == ".cmd" {
        format!("@echo off\r\n{}", script.script)
    } else {
        script.script.clone()
    };
    let path = std::env::temp_dir().join(format!(
        "clippy-command-{}{extension}",
        uuid::Uuid::now_v7()
    ));
    tokio::fs::write(&path, body)
        .await
        .map_err(|e| e.to_string())?;

    let input = get_app()
        .state::<Clipboard>()
        .read_text()
        .unwrap_or_default();
    let result = execute(&program, &args, &path, input).await;

    let _ = tokio::fs::remove_file(&path).await;
    result
}

fn resolve_interpreter(interpreter: &str) -> (String, Vec<String>, &'static str) {
    let mut parts = interpreter.split_whitespace().map(str::to_string);
    let program = parts.next().unwrap_or_else(|| {
        if cfg!(target_os = "windows") {
            "powershell".to_string()
        } else {
            "sh".to_string()
        }
    });
    let mut args: Vec<String> = parts.collect();

    let name = Path::new(&program)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    let extension = match name.as_str() {
        "powershell" | "pwsh" => {
            args.extend(
                [
                    "-NoProfile",
                    "-NonInteractive",
                    "-ExecutionPolicy",
                    "Bypass",
                    "-File",
                ]
                .map(str::to_string),
            );
            ".ps1"
        }
        "cmd" => {
            args.extend(["/D", "/C"].map(str::to_string));
            ".cmd"
        }
        _ => "",
    };

    (program, args, extension)
}

async fn execute(
    program: &str,
    args: &[String],
    path: &Path,
    input: String,
) -> Result<String, String> {
    let mut command = tokio::process::Command::new(program);
    command
        .args(args)
        .arg(path)
        .env(
            "CLIPPY_TEXT",
            if input.len() <= MAX_ENV_TEXT_LEN {
                input.as_str()
            } else {
                ""
            },
        )
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);

    #[cfg(target_os = "windows")]
    command.creation_flags(CREATE_NO_WINDOW);

    let mut child = command.spawn().map_err(|e| format!("{program}: {e}"))?;

    if let Some(mut stdin) = child.stdin.take() {
        tokio::spawn(async move {
            let _ = stdin.write_all(input.as_bytes()).await;
        });
    }

    let output = tokio::time::timeout(COMMAND_TIMEOUT, child.wait_with_output())
        .await
        .map_err(|_| format!("timed out after {}s", COMMAND_TIMEOUT.as_secs()))?
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    Err(if stderr.is_empty() {
        output.status.to_string()
    } else {
        stderr
    })
}

async fn apply_output(command: &CustomCommand, stdout: &str) {
    let output = CommandOutput::from_setting(&command.output);
    if output == CommandOutput::Ignore {
        return;
    }

    // `echo` and friends end with a newline nobody wants pasted.
    let text = stdout
        .strip_suffix('\n')
        .map(|text| text.strip_suffix('\r').unwrap_or(text))
        .unwrap_or(stdout);
    if text.is_empty() {
        return;
    }

    if let Err(e) = get_app().state::<Clipboard>().write_text(text.to_string()) {
        notify_failure(&command.name, &e);
        return;
    }

    if output == CommandOutput::Paste {
        paste_into_active_window().await;
    }
}

fn notify_failure(name: &str, error: &str) {
    printlog!("command {name:?} failed: {error}");

    let title = if name.is_empty() { "Clippy" } else { name };
    if let Err(e) = get_app()
        .notification()
        .builder()
        .title(title)
        .body(error)
        .show()
    {
        printlog!("command notification failed: {e:?}");
    }
}
