use chrono::NaiveDateTime;
use common::{
    constants::{BACKDUP_DATE_FORMAT, BACKUP_FILE_PREFIX},
    types::sync::ClippyInfo,
};
use sea_orm::prelude::Uuid;

pub mod google_drive;

pub fn parse_clipboard_info(filename: &str, provider_id: &String) -> Option<ClippyInfo> {
    let [_, uuid, star, created_at, deleted_at]: [&str; 5] = filename
        .trim_end_matches(".json")
        .split('_')
        .collect::<Vec<_>>()
        .try_into()
        .ok()?;

    let id = Uuid::parse_str(uuid).ok()?;
    let starred = star.parse().ok()?;
    let created_date = NaiveDateTime::parse_from_str(created_at, BACKDUP_DATE_FORMAT).ok()?;
    let deleted_date = if deleted_at == "None" {
        None
    } else {
        Some(NaiveDateTime::parse_from_str(deleted_at, BACKDUP_DATE_FORMAT).ok()?)
    };

    Some(ClippyInfo {
        id,
        starred,
        created_at: created_date,
        deleted_at: deleted_date,
        provider_id: provider_id.clone(),
    })
}

pub fn create_clipboard_filename(id: &Uuid, starred: &bool, created_at: &NaiveDateTime) -> String {
    format!(
        "{}_{}_{}_{}_{}.json",
        BACKUP_FILE_PREFIX,
        id,
        starred,
        created_at.format(BACKDUP_DATE_FORMAT),
        "None"
    )
}
