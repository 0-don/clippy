use sea_orm::EnumIter;
pub use sea_orm_migration::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

mod m000001_create_clipboard;
mod m000002_create_clipboard_text;
mod m000003_create_clipboard_image;
mod m000004_create_clipboard_html;
mod m000005_create_clipboard_rtf;
mod m000006_create_clipboard_file;

mod m000007_create_settings;
mod m000008_create_hotkey;
mod m000009_seed;

pub struct Migrator;

#[derive(Iden, EnumIter, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum CommandEvent {
    #[iden = "init"]
    Init,
    #[iden = "set_global_hotkey_event"]
    SetGlobalHotkeyEvent,
    #[iden = "change_tab"]
    ChangeTab,
    #[iden = "open_window"]
    OpenWindow,
}

#[derive(Iden, EnumIter, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum HotkeyEvent {
    #[iden = "window_display_toggle"]
    WindowDisplayToggle,
    #[iden = "type_clipboard"]
    TypeClipboard,
    #[iden = "scroll_to_top"]
    ScrollToTop,
    #[iden = "sync_clipboard_history"]
    SyncClipboardHistory,
    #[iden = "preferences"]
    Preferences,
    #[iden = "about"]
    About,
    #[iden = "exit"]
    Exit,
    #[iden = "recent_clipboards"]
    RecentClipboard,
    #[iden = "starred_clipboards"]
    StarredClipboard,
    #[iden = "history"]
    History,
    #[iden = "view_more"]
    ViewMore,
    #[iden = "digit_1"]
    Digit1,
    #[iden = "digit_2"]
    Digit2,
    #[iden = "digit_3"]
    Digit3,
    #[iden = "digit_4"]
    Digit4,
    #[iden = "digit_5"]
    Digit5,
    #[iden = "digit_6"]
    Digit6,
    #[iden = "digit_7"]
    Digit7,
    #[iden = "digit_8"]
    Digit8,
    #[iden = "digit_9"]
    Digit9,
    #[iden = "num_1"]
    Num1,
    #[iden = "num_2"]
    Num2,
    #[iden = "num_3"]
    Num3,
    #[iden = "num_4"]
    Num4,
    #[iden = "num_5"]
    Num5,
    #[iden = "num_6"]
    Num6,
    #[iden = "num_7"]
    Num7,
    #[iden = "num_8"]
    Num8,
    #[iden = "num_9"]
    Num9,
}

#[derive(Iden, EnumIter, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum ClipboardTextType {
    #[iden = "text"]
    Text,
    #[iden = "link"]
    Link,
    #[iden = "hex"]
    Hex,
    #[iden = "rgb"]
    Rgb,
}
#[derive(Iden, EnumIter, PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum ClipboardType {
    #[iden = "text"]
    Text,
    #[iden = "image"]
    Image,
    #[iden = "html"]
    Html,
    #[iden = "rtf"]
    Rtf,
    #[iden = "file"]
    File,
}

impl ClipboardType {
    pub fn from_json_value(value: &JsonValue) -> Option<Vec<Self>> {
        match value {
            JsonValue::Array(arr) => {
                let types: Vec<ClipboardType> = arr
                    .iter()
                    .filter_map(|v| match v {
                        JsonValue::String(s) => match s.as_str() {
                            s if s == Self::Text.to_string() => Some(Self::Text),
                            s if s == Self::Image.to_string() => Some(Self::Image),
                            s if s == Self::Html.to_string() => Some(Self::Html),
                            s if s == Self::Rtf.to_string() => Some(Self::Rtf),
                            s if s == Self::File.to_string() => Some(Self::File),
                            _ => None,
                        },
                        _ => None,
                    })
                    .collect();

                if types.is_empty() {
                    None
                } else {
                    Some(types)
                }
            }
            _ => None,
        }
    }

    pub fn to_json_value(types: &Vec<Self>) -> JsonValue {
        json!(types.iter().map(|t| t.to_string()).collect::<Vec<_>>())
    }
}

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m000001_create_clipboard::Migration),
            Box::new(m000002_create_clipboard_text::Migration),
            Box::new(m000003_create_clipboard_image::Migration),
            Box::new(m000004_create_clipboard_html::Migration),
            Box::new(m000005_create_clipboard_rtf::Migration),
            Box::new(m000006_create_clipboard_file::Migration),
            Box::new(m000007_create_settings::Migration),
            Box::new(m000008_create_hotkey::Migration),
            Box::new(m000009_seed::Migration),
        ]
    }
}
