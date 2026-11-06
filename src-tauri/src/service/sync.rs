use super::settings::{
    get_global_settings, get_settings_db, init_settings_window, update_settings_from_sync,
    update_settings_synchronize_db,
};
use crate::{
    prelude::*,
    tao::{config::get_data_path, global::get_app},
    utils::{providers::google_drive::GoogleDriveProviderImpl, sync_manager::SyncManager},
};
use common::{
    constants::TOKEN_NAME,
    types::{
        enums::{ListenEvent, SyncProviderType},
        orm_query::FullClipboardDto,
        sync::{Clippy, SyncProvider, SyncState, SyncStatus},
        types::CommandError,
    },
};
use entity::settings;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, LazyLock,
    },
};
use tauri::{Emitter, EventTarget, Manager, State};
use tokio::sync::Mutex;

static SYNC_PROVIDER: LazyLock<Mutex<Option<Arc<dyn SyncProvider>>>> =
    LazyLock::new(|| Mutex::new(None));
// Bumped on disconnect so an auth flow still in flight cannot store its provider afterwards.
static SYNC_PROVIDER_GENERATION: AtomicU64 = AtomicU64::new(0);
static SYNC_STATUS: LazyLock<std::sync::Mutex<SyncStatus>> =
    LazyLock::new(|| std::sync::Mutex::new(SyncStatus::default()));

pub fn get_sync_status() -> SyncStatus {
    SYNC_STATUS.lock().expect("sync status poisoned").clone()
}

pub fn update_sync_status(update: impl FnOnce(&mut SyncStatus)) {
    let status = {
        let mut status = SYNC_STATUS.lock().expect("sync status poisoned");
        update(&mut status);
        status.clone()
    };
    get_app()
        .emit_to(
            EventTarget::any(),
            ListenEvent::SyncStatus.to_string().as_str(),
            status,
        )
        .ok();
}

pub async fn upsert_settings_sync(
    settings: &settings::Model,
    block_main_thread: bool,
) -> Result<(), CommandError> {
    if settings.sync {
        let settings_map: HashMap<String, serde_json::Value> =
            serde_json::from_value(serde_json::to_value(settings.clone())?)?;
        if block_main_thread {
            get_sync_provider()
                .await
                .upsert_settings(&settings_map)
                .await
                .expect("Failed to upsert settings");
        } else {
            tauri::async_runtime::spawn(async move {
                get_sync_provider()
                    .await
                    .upsert_settings(&settings_map)
                    .await
                    .expect("Failed to upsert settings");
            });
        }
    }
    Ok(())
}

pub async fn get_sync_provider() -> Arc<dyn SyncProvider> {
    if let Some(provider) = SYNC_PROVIDER.lock().await.as_ref() {
        return provider.clone();
    }

    let generation = SYNC_PROVIDER_GENERATION.load(Ordering::SeqCst);
    update_sync_status(|status| {
        if status.state == SyncState::Disabled {
            status.state = SyncState::Connecting;
        }
    });

    // Built without holding the lock: the browser login can take minutes and a
    // disconnect must not wait on it.
    let provider: Arc<dyn SyncProvider> = match get_global_settings().sync_provider.as_str() {
        s if s == SyncProviderType::GoogleDrive.to_string() => Arc::new(
            GoogleDriveProviderImpl::new()
                .await
                .expect("Failed to create Google Drive provider"),
        ),
        _ => panic!("Provider type not implemented"),
    };

    let mut slot = SYNC_PROVIDER.lock().await;
    if let Some(existing) = slot.as_ref() {
        return existing.clone();
    }
    if SYNC_PROVIDER_GENERATION.load(Ordering::SeqCst) == generation {
        *slot = Some(provider.clone());
    }
    provider
}

pub fn get_sync_manager() -> State<'static, Mutex<SyncManager>> {
    match get_app().try_state() {
        Some(manager) => manager,
        None => {
            let manager = Mutex::new(SyncManager::new());
            get_app().manage(manager);
            get_app().state()
        }
    }
}

pub fn setup_sync_interval() {
    tauri::async_runtime::spawn(async {
        // needs to be real db call because of setup race condition in `fn setup_settings()`
        let settings = get_settings_db().await.expect("Failed to get settings");

        if settings.sync {
            get_sync_manager().lock().await.start().await;
        }
    });
}

pub async fn sync_toggle() -> Result<bool, CommandError> {
    let new_sync_state = !get_global_settings().sync;

    if new_sync_state {
        // Trying to enable sync
        let provider = get_sync_provider().await;
        if !provider.is_authenticated().await {
            update_settings_synchronize_db(false).await?;
            update_sync_status(|status| *status = SyncStatus::default());
            return Err(CommandError::Error("Authentication failed".to_string()));
        }

        let remote_settings = provider
            .get_settings()
            .await
            .expect("Failed to get settings");
        update_settings_from_sync(remote_settings)
            .await
            .expect("Failed to update settings");

        let settings = update_settings_synchronize_db(true).await?;

        upsert_settings_sync(&settings, false).await?;

        // Don't stop existing sync if enabling - just start a new one
        get_sync_manager().lock().await.start().await;
    } else {
        // Trying to disable sync
        get_sync_manager().lock().await.stop().await;
        let settings = update_settings_synchronize_db(false).await?;
        upsert_settings_sync(&settings, false).await?;
        update_sync_status(|status| *status = SyncStatus::default());
    }

    Ok(new_sync_state)
}

pub async fn sync_disconnect() -> Result<(), CommandError> {
    get_sync_manager().lock().await.stop().await;
    update_settings_synchronize_db(false).await?;

    SYNC_PROVIDER_GENERATION.fetch_add(1, Ordering::SeqCst);
    SYNC_PROVIDER.lock().await.take();

    let token_path = std::path::Path::new(&get_data_path().config_path).join(TOKEN_NAME);
    if token_path.exists() {
        std::fs::remove_file(token_path)?;
    }

    update_sync_status(|status| *status = SyncStatus::default());
    init_settings_window();
    Ok(())
}

pub async fn sync_reconnect() -> Result<bool, CommandError> {
    sync_disconnect().await?;
    sync_toggle().await
}

/// Re-uploads changed clipboards a few at a time; one Drive round trip per
/// clipboard made bulk encrypt/decrypt take minutes when done serially.
pub async fn update_remote_clipboards(
    provider: Arc<dyn SyncProvider>,
    updates: Vec<(FullClipboardDto, Clippy)>,
) {
    const PARALLEL_UPLOADS: usize = 8;
    let mut tasks = tokio::task::JoinSet::new();

    for (local, remote) in updates {
        while tasks.len() >= PARALLEL_UPLOADS {
            tasks.join_next().await;
        }
        let provider = provider.clone();
        tasks.spawn(async move {
            if let Err(e) = provider.update_clipboard(&local, &remote).await {
                printlog!("Failed to update remote clipboard {}: {:?}", remote.id, e);
            }
        });
    }

    while tasks.join_next().await.is_some() {}
}
