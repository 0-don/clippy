use super::custom_command::run_remote_command;
use crate::{prelude::*, tao::config::get_data_path};
use common::{constants::DEVICE_KEY_NAME, types::sync::DeviceInfo};
use iroh::{
    endpoint::{presets, Connection},
    protocol::{AcceptError, ProtocolHandler, Router},
    Endpoint, EndpointId, SecretKey,
};
use serde::{Deserialize, Serialize};
use std::{
    path::Path,
    sync::{LazyLock, Mutex, OnceLock},
    time::Duration,
};

const ALPN: &[u8] = b"clippy/command/1";
const SEND_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_MESSAGE_BYTES: usize = 1024 * 1024;

static SECRET_KEY: OnceLock<SecretKey> = OnceLock::new();
static ROUTER: OnceLock<Router> = OnceLock::new();
/// Devices published to the sync account; the only peers allowed to send commands.
static DEVICES: LazyLock<Mutex<Vec<DeviceInfo>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[derive(Serialize, Deserialize)]
struct RemoteCommand {
    command_id: String,
    text: String,
}

/// Per-device identity, kept next to the config and never synced.
fn secret_key() -> &'static SecretKey {
    SECRET_KEY.get_or_init(|| {
        let path = Path::new(&get_data_path().config_path).join(DEVICE_KEY_NAME);
        if let Some(bytes) = std::fs::read(&path)
            .ok()
            .and_then(|bytes| <[u8; 32]>::try_from(bytes.as_slice()).ok())
        {
            return SecretKey::from_bytes(&bytes);
        }

        let key = SecretKey::generate();
        if let Err(e) = write_private(&path, &key.to_bytes()) {
            printlog!("p2p: failed to store device key: {}", e);
        }
        key
    })
}

fn write_private(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::mode(&mut options, 0o600);
    options.open(path)?.write_all(bytes)
}

pub fn device_id() -> String {
    secret_key().public().to_string()
}

pub fn this_device() -> DeviceInfo {
    DeviceInfo {
        id: device_id(),
        name: gethostname::gethostname().to_string_lossy().into_owned(),
        os: std::env::consts::OS.to_string(),
        last_seen: chrono::Utc::now().timestamp(),
    }
}

pub fn set_devices(devices: Vec<DeviceInfo>) {
    *DEVICES.lock().expect("devices poisoned") = devices;
}

pub fn devices() -> Vec<DeviceInfo> {
    DEVICES.lock().expect("devices poisoned").clone()
}

pub fn setup_p2p() {
    tauri::async_runtime::spawn(async {
        let endpoint = match Endpoint::builder(presets::N0)
            .secret_key(secret_key().clone())
            .bind()
            .await
        {
            Ok(endpoint) => endpoint,
            Err(e) => {
                printlog!("p2p: failed to start endpoint: {:?}", e);
                return;
            }
        };
        let router = Router::builder(endpoint)
            .accept(ALPN, CommandHandler)
            .spawn();
        printlog!("p2p: listening as {}", device_id());
        ROUTER.set(router).ok();
    });
}

/// Asks every other known device to run its own copy of the command. Devices that
/// are offline or unreachable within the timeout are skipped, never retried later.
pub async fn send_command_to_others(command_id: String, text: String) {
    let Some(router) = ROUTER.get() else {
        printlog!("p2p: endpoint not ready, command not sent");
        return;
    };
    let me = device_id();
    let Ok(payload) = serde_json::to_vec(&RemoteCommand { command_id, text }) else {
        return;
    };

    let mut tasks = tokio::task::JoinSet::new();
    for device in devices().into_iter().filter(|device| device.id != me) {
        let Ok(id) = device.id.parse::<EndpointId>() else {
            continue;
        };
        let endpoint = router.endpoint().clone();
        let payload = payload.clone();
        tasks.spawn(async move {
            let sent = tokio::time::timeout(SEND_TIMEOUT, async {
                let connection = endpoint
                    .connect(id, ALPN)
                    .await
                    .map_err(|e| e.to_string())?;
                let mut send = connection.open_uni().await.map_err(|e| e.to_string())?;
                send.write_all(&payload).await.map_err(|e| e.to_string())?;
                send.finish().map_err(|e| e.to_string())?;
                // The receiver closes once it has read the message.
                connection.closed().await;
                Ok::<(), String>(())
            })
            .await;
            match sent {
                Ok(Ok(())) => printlog!("p2p: command sent to {}", device.name),
                Ok(Err(e)) => printlog!("p2p: failed to reach {}: {}", device.name, e),
                Err(_) => printlog!("p2p: {} did not answer in time", device.name),
            }
        });
    }
    while tasks.join_next().await.is_some() {}
}

#[derive(Debug, Clone)]
struct CommandHandler;

impl ProtocolHandler for CommandHandler {
    async fn accept(&self, connection: Connection) -> Result<(), AcceptError> {
        let remote = connection.remote_id().to_string();
        let trusted = remote != device_id() && devices().iter().any(|device| device.id == remote);
        if !trusted {
            printlog!("p2p: rejected unknown device {}", remote);
            connection.close(1u32.into(), b"unknown device");
            return Ok(());
        }

        let mut recv = connection.accept_uni().await?;
        let bytes = recv
            .read_to_end(MAX_MESSAGE_BYTES)
            .await
            .map_err(AcceptError::from_err)?;
        connection.close(0u32.into(), b"ok");

        if let Ok(message) = serde_json::from_slice::<RemoteCommand>(&bytes) {
            tauri::async_runtime::spawn(run_remote_command(message.command_id, message.text));
        }
        Ok(())
    }
}
