use chrono::{DateTime, NaiveDateTime};
use common::{
    constants::{BACKDUP_DATE_FORMAT, BACKUP_FILE_PREFIX},
    types::sync::Clippy,
};
use sea_orm::prelude::Uuid;
pub mod google_drive;

pub fn uuid_to_datetime(uuid: &Uuid) -> NaiveDateTime {
    let ts = uuid.get_timestamp().expect("Not a time-based UUID");
    let (secs, nanos) = ts.to_unix();
    DateTime::from_timestamp(secs as i64, nanos as u32)
        .expect("Invalid timestamp")
        .naive_utc()
}

pub fn parse_clipboard_info(filename: &str, provider_id: &String) -> Option<Clippy> {
    let [_, uuid, star, encrypted, created_at, deleted_at]: [&str; 6] = filename
        .trim_end_matches(".json")
        .split('_')
        .collect::<Vec<_>>()
        .try_into()
        .ok()?;

    let id = Uuid::parse_str(uuid).ok()?;
    let starred = star.parse().ok()?;
    let encrypted = encrypted.parse().ok()?;
    let created_at = NaiveDateTime::parse_from_str(created_at, BACKDUP_DATE_FORMAT).ok()?;
    let deleted_at = if deleted_at == "None" {
        None
    } else {
        Some(NaiveDateTime::parse_from_str(deleted_at, BACKDUP_DATE_FORMAT).ok()?)
    };

    Some(Clippy {
        id,
        star: starred,
        encrypted,
        created_at,
        deleted_at,
        provider_id: provider_id.clone(),
    })
}

pub fn create_clipboard_filename(
    id: &Uuid,
    starred: &bool,
    encrypted: &bool,
    created_at: &NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
) -> String {
    format!(
        "{}_{}_{}_{}_{}_{}.json",
        BACKUP_FILE_PREFIX,
        id,
        starred,
        encrypted,
        created_at.format(BACKDUP_DATE_FORMAT),
        deleted_at
            .map(|date| date.format(BACKDUP_DATE_FORMAT).to_string())
            .unwrap_or_else(|| "None".to_string())
    )
}
