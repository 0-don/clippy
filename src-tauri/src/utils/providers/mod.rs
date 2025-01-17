use chrono::NaiveDateTime;
use common::{constants::BACKUP_FILE_PREFIX, types::sync::ClippyInfo};
use sea_orm::prelude::Uuid;

pub mod google_drive;

pub fn parse_clipboard_info(filename: &str, provider_id: &str) -> Option<ClippyInfo> {
    let [_, timestamp, star, uuid]: [&str; 4] =
        filename.split('_').collect::<Vec<_>>().try_into().ok()?;

    let id = Uuid::parse_str(uuid.trim_end_matches(".json")).ok()?;
    let starred = star.parse().ok()?;
    let timestamp = NaiveDateTime::parse_from_str(timestamp, "%Y%m%d%H%M%S").ok()?;

    Some(ClippyInfo {
        id,
        starred,
        timestamp,
        provider_id: provider_id.to_string(),
    })
}

pub fn create_clipboard_filename(created_date: &NaiveDateTime, star: bool, id: &Uuid) -> String {
    format!(
        "{}_{}_{}_{}.json",
        BACKUP_FILE_PREFIX,
        created_date.format("%Y%m%d%H%M%S"),
        star,
        id
    )
}
