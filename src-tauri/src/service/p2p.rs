use super::{
    custom_command::run_remote_command,
    decrypt::{decrypt_data_with_key, read_encryption_key},
    encrypt::encrypt_data,
    settings::get_global_settings,
    sync::{current_sync_provider, update_sync_status},
};
use crate::{prelude::*, service::cipher::is_encryption_key_set, tao::config::get_data_path};
use base64::{engine::general_purpose::STANDARD, Engine};
use common::{
    constants::DEVICE_KEY_NAME,
    types::{enums::CommandTarget, sync::DeviceInfo, types::CustomCommand},
};
use iroh::{
    address_lookup::memory::MemoryLookup,
    endpoint::{presets, Connection},
    protocol::{AcceptError, ProtocolHandler, Router},
    tls::CaTlsConfig,
    Endpoint, EndpointAddr, EndpointId, SecretKey,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::Path,
    sync::{LazyLock, Mutex, OnceLock},
    time::Duration,
};

const ALPN: &[u8] = b"clippy/command/1";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(4);
const SEND_ATTEMPTS: usize = 2;
const MAX_MESSAGE_BYTES: usize = 1024 * 1024;
/// Commands left in Drive older than this are dropped instead of run late.
const DRIVE_COMMAND_MAX_AGE_SECS: i64 = 60;
const DRIVE_POLL_INTERVAL: Duration = Duration::from_secs(3);
/// A device that has not republished within this window is shown as offline.
const DEVICE_STALE_SECS: i64 = 6 * 60;

static SECRET_KEY: OnceLock<SecretKey> = OnceLock::new();
static ROUTER: OnceLock<Router> = OnceLock::new();
/// Addresses published in Drive; dialing through this works when DNS lookup is blocked.
static ADDRESS_BOOK: LazyLock<MemoryLookup> = LazyLock::new(MemoryLookup::new);
/// Devices published to the sync account; the only peers allowed to send commands.
static DEVICES: LazyLock<Mutex<Vec<DeviceInfo>>> = LazyLock::new(|| Mutex::new(Vec::new()));
static PEER_STATUS: LazyLock<Mutex<HashMap<String, PeerStatus>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PeerLink {
    Direct,
    Relay,
    Drive,
    Offline,
}

#[derive(Clone, Debug, Serialize)]
pub struct PeerStatus {
    pub link: PeerLink,
    pub rtt_ms: Option<u64>,
}

/// Same shape as the 1.7.12 message, so older receivers still accept commands.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Message {
    Command { command_id: String, text: String },
    Ping { ping: bool },
}

#[derive(Serialize, Deserialize)]
struct DriveCommand {
    command_id: String,
    text: String,
    encrypted: bool,
    from: String,
    sent_at: i64,
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
        addr: ROUTER
            .get()
            .and_then(|router| serde_json::to_value(router.endpoint().addr()).ok()),
    }
}

pub fn set_devices(devices: Vec<DeviceInfo>) {
    for device in &devices {
        if let Some(addr) = device
            .addr
            .clone()
            .and_then(|addr| serde_json::from_value::<EndpointAddr>(addr).ok())
        {
            ADDRESS_BOOK.set_endpoint_info(addr);
        }
    }
    *DEVICES.lock().expect("devices poisoned") = devices;
}

pub fn devices() -> Vec<DeviceInfo> {
    DEVICES.lock().expect("devices poisoned").clone()
}

pub fn peer_statuses() -> HashMap<String, PeerStatus> {
    PEER_STATUS.lock().expect("peer status poisoned").clone()
}

/// Whether this device holds a relay connection, i.e. others can reach it over P2P.
pub fn is_online() -> bool {
    use iroh::Watcher;
    ROUTER.get().is_some_and(|router| {
        router
            .endpoint()
            .home_relay_status()
            .get()
            .iter()
            .any(|relay| relay.is_connected())
    })
}

pub fn setup_p2p() {
    tauri::async_runtime::spawn(async {
        let mut backoff = Duration::from_secs(5);
        let endpoint = loop {
            let bound = Endpoint::builder(presets::N0)
                .secret_key(secret_key().clone())
                .address_lookup(ADDRESS_BOOK.clone())
                // Corporate networks: route relays through the proxy, trust the OS CA store.
                .proxy_from_env()
                .ca_tls_config(CaTlsConfig::system())
                .bind()
                .await;
            match bound {
                Ok(endpoint) => break endpoint,
                Err(e) => {
                    printlog!(
                        "p2p: failed to start endpoint, retrying in {:?}: {:?}",
                        backoff,
                        e
                    );
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(Duration::from_secs(60));
                }
            }
        };
        let router = Router::builder(endpoint)
            .accept(ALPN, CommandHandler)
            .spawn();
        printlog!("p2p: listening as {}", device_id());
        ROUTER.set(router).ok();
    });
    tauri::async_runtime::spawn(poll_drive_commands());
}

fn parse_id(device: &DeviceInfo) -> Option<EndpointId> {
    device.id.parse::<EndpointId>().ok()
}

/// Opens a connection and delivers one message; returns how the peer was reached.
async fn deliver(
    endpoint: &Endpoint,
    id: EndpointId,
    payload: &[u8],
) -> Result<PeerStatus, String> {
    tokio::time::timeout(CONNECT_TIMEOUT, async {
        let connection = endpoint
            .connect(id, ALPN)
            .await
            .map_err(|e| e.to_string())?;
        let status = link_of(&connection);
        let mut send = connection.open_uni().await.map_err(|e| e.to_string())?;
        send.write_all(payload).await.map_err(|e| e.to_string())?;
        send.finish().map_err(|e| e.to_string())?;
        // The receiver closes once it has read the message.
        connection.closed().await;
        Ok(status)
    })
    .await
    .map_err(|_| "timed out".to_string())?
}

fn link_of(connection: &Connection) -> PeerStatus {
    let paths = connection.paths();
    let selected = paths.iter().find(|path| path.is_selected());
    PeerStatus {
        link: match &selected {
            Some(path) if path.is_ip() => PeerLink::Direct,
            _ => PeerLink::Relay,
        },
        rtt_ms: selected.map(|path| path.rtt().as_millis() as u64),
    }
}

fn set_peer_status(id: &str, status: PeerStatus) {
    PEER_STATUS
        .lock()
        .expect("peer status poisoned")
        .insert(id.to_string(), status);
}

fn fallback_status(device: &DeviceInfo) -> PeerStatus {
    let fresh = chrono::Utc::now().timestamp() - device.last_seen <= DEVICE_STALE_SECS;
    PeerStatus {
        link: if fresh {
            PeerLink::Drive
        } else {
            PeerLink::Offline
        },
        rtt_ms: None,
    }
}

fn other_devices() -> Vec<DeviceInfo> {
    let me = device_id();
    devices()
        .into_iter()
        .filter(|device| device.id != me)
        .collect()
}

/// Asks every other known device to run its own copy of the command: directly over
/// iroh when possible, otherwise through a short lived file in the sync account.
pub async fn send_command_to_others(command_id: String, text: String) {
    let Ok(payload) = serde_json::to_vec(&Message::Command {
        command_id: command_id.clone(),
        text: text.clone(),
    }) else {
        return;
    };

    let mut tasks = tokio::task::JoinSet::new();
    for device in other_devices() {
        let endpoint = ROUTER.get().map(|router| router.endpoint().clone());
        let payload = payload.clone();
        let (command_id, text) = (command_id.clone(), text.clone());
        tasks.spawn(async move {
            if let (Some(endpoint), Some(id)) = (endpoint, parse_id(&device)) {
                for attempt in 1..=SEND_ATTEMPTS {
                    match deliver(&endpoint, id, &payload).await {
                        Ok(status) => {
                            printlog!("p2p: command sent to {} ({:?})", device.name, status.link);
                            set_peer_status(&device.id, status);
                            return;
                        }
                        Err(e) => printlog!(
                            "p2p: attempt {} to reach {} failed: {}",
                            attempt,
                            device.name,
                            e
                        ),
                    }
                }
            }
            send_via_drive(&device, command_id, text).await;
        });
    }
    while tasks.join_next().await.is_some() {}
}

async fn send_via_drive(device: &DeviceInfo, command_id: String, text: String) {
    let Some(provider) = current_sync_provider().await else {
        printlog!(
            "p2p: {} unreachable and sync is off, command dropped",
            device.name
        );
        return;
    };
    // Clipboard text in the sync account follows the same encryption as clipboards.
    let (text, encrypted) = if get_global_settings().encryption && is_encryption_key_set() {
        match encrypt_data(text.as_bytes()) {
            Ok(data) => (STANDARD.encode(data), true),
            Err(_) => return,
        }
    } else {
        (text, false)
    };
    let command = DriveCommand {
        command_id,
        text,
        encrypted,
        from: device_id(),
        sent_at: chrono::Utc::now().timestamp(),
    };
    let Ok(payload) = serde_json::to_string(&command) else {
        return;
    };
    match provider.push_remote_command(&device.id, &payload).await {
        Ok(()) => {
            printlog!("p2p: command for {} left in Google Drive", device.name);
            set_peer_status(&device.id, fallback_status(device));
        }
        Err(e) => printlog!("p2p: failed to leave command for {}: {:?}", device.name, e),
    }
}

fn uses_remote_commands() -> bool {
    CustomCommand::from_json_value(&get_global_settings().commands)
        .iter()
        .any(|command| CommandTarget::from_setting(&command.target) != CommandTarget::Local)
}

/// Picks up commands other devices left in Drive while P2P was unavailable. Only
/// runs while some command targets other devices, so idle installs cost nothing.
async fn poll_drive_commands() {
    loop {
        tokio::time::sleep(DRIVE_POLL_INTERVAL).await;
        if !get_global_settings().sync || !uses_remote_commands() {
            continue;
        }
        let Some(provider) = current_sync_provider().await else {
            continue;
        };
        let Ok(payloads) = provider.take_remote_commands(&device_id()).await else {
            continue;
        };

        let trusted: Vec<String> = devices().into_iter().map(|device| device.id).collect();
        for payload in payloads {
            let Ok(command) = serde_json::from_str::<DriveCommand>(&payload) else {
                continue;
            };
            let age = chrono::Utc::now().timestamp() - command.sent_at;
            if age > DRIVE_COMMAND_MAX_AGE_SECS || !trusted.contains(&command.from) {
                printlog!("p2p: dropped Drive command ({}s old)", age);
                continue;
            }
            let text = if command.encrypted {
                let decrypted = STANDARD
                    .decode(&command.text)
                    .ok()
                    .zip(read_encryption_key().ok())
                    .and_then(|(data, key)| decrypt_data_with_key(&data, &key).ok());
                match decrypted {
                    Some(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                    None => {
                        printlog!("p2p: could not decrypt Drive command, skipped");
                        continue;
                    }
                }
            } else {
                command.text
            };
            tauri::async_runtime::spawn(run_remote_command(command.command_id, text));
        }
    }
}

/// Measures how every other device can be reached and publishes the result to the UI.
pub async fn probe_devices() {
    let Some(endpoint) = ROUTER.get().map(|router| router.endpoint().clone()) else {
        return;
    };
    let Ok(ping) = serde_json::to_vec(&Message::Ping { ping: true }) else {
        return;
    };

    let mut tasks = tokio::task::JoinSet::new();
    for device in other_devices() {
        let (endpoint, ping) = (endpoint.clone(), ping.clone());
        tasks.spawn(async move {
            let status = match parse_id(&device) {
                Some(id) => deliver(&endpoint, id, &ping)
                    .await
                    .unwrap_or_else(|_| fallback_status(&device)),
                None => fallback_status(&device),
            };
            set_peer_status(&device.id, status);
        });
    }
    while tasks.join_next().await.is_some() {}
    update_sync_status(|_| {});
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

        if let Ok(Message::Command { command_id, text }) = serde_json::from_slice(&bytes) {
            tauri::async_runtime::spawn(run_remote_command(command_id, text));
        }
        Ok(())
    }
}
