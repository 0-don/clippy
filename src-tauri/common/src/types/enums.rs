use sea_orm::prelude::*;
use sea_orm::EnumIter;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

#[derive(DeriveIden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ClippyPosition {
    #[sea_orm(iden = "cursor")]
    Cursor,
    #[sea_orm(iden = "top_left")]
    TopLeft,
    #[sea_orm(iden = "top_right")]
    TopRight,
    #[sea_orm(iden = "bottom_left")]
    BottomLeft,
    #[sea_orm(iden = "bottom_right")]
    BottomRight,
    #[sea_orm(iden = "top_center")]
    TopCenter,
    #[sea_orm(iden = "bottom_center")]
    BottomCenter,
    #[sea_orm(iden = "left_center")]
    LeftCenter,
    #[sea_orm(iden = "right_center")]
    RightCenter,
    #[sea_orm(iden = "center")]
    Center,
    // #[sea_orm(iden = "tray_left")]
    // TrayLeft,
    // #[sea_orm(iden = "tray_bottom_left")]
    // TrayBottomLeft,
    // #[sea_orm(iden = "tray_right")]
    // TrayRight,
    // #[sea_orm(iden = "tray_bottom_right")]
    // TrayBottomRight,
    // #[sea_orm(iden = "tray_center")]
    // TrayCenter,
    // #[sea_orm(iden = "tray_bottom_center")]
    // TrayBottomCenter,
}

#[derive(DeriveIden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PasteOnSelect {
    #[sea_orm(iden = "off")]
    Off,
    #[sea_orm(iden = "paste")]
    Paste,
    #[sea_orm(iden = "type")]
    Type,
}

impl PasteOnSelect {
    /// Anything unrecognised means Off: a settings row synced from a newer
    /// client can name a mode this build has no code for, and silently sending
    /// keystrokes is worse than doing nothing.
    pub fn from_setting(value: &str) -> Self {
        match value {
            "paste" => Self::Paste,
            "type" => Self::Type,
            _ => Self::Off,
        }
    }
}

#[derive(DeriveIden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CommandOutput {
    #[sea_orm(iden = "ignore")]
    Ignore,
    #[sea_orm(iden = "copy")]
    Copy,
    #[sea_orm(iden = "paste")]
    Paste,
}

impl CommandOutput {
    /// Unknown modes from a newer synced client do nothing rather than send keystrokes.
    pub fn from_setting(value: &str) -> Self {
        match value {
            "copy" => Self::Copy,
            "paste" => Self::Paste,
            _ => Self::Ignore,
        }
    }
}

#[derive(DeriveIden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum SyncProviderType {
    #[sea_orm(iden = "google_drive")]
    GoogleDrive,
}

#[derive(DeriveIden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FolderLocation {
    #[sea_orm(iden = "database")]
    Database,
    #[sea_orm(iden = "config")]
    Config,
}

#[derive(DeriveIden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[sea_orm(iden = "en")]
    English,
    #[sea_orm(iden = "zh")]
    Mandarin,
    #[sea_orm(iden = "hi")]
    Hindi,
    #[sea_orm(iden = "es")]
    Spanish,
    #[sea_orm(iden = "fr")]
    French,
    #[sea_orm(iden = "ar")]
    Arabic,
    #[sea_orm(iden = "bn")]
    Bengali,
    #[sea_orm(iden = "pt")]
    Portuguese,
    #[sea_orm(iden = "ru")]
    Russian,
    #[sea_orm(iden = "ur")]
    Urdu,
    #[sea_orm(iden = "ja")]
    Japanese,
    #[sea_orm(iden = "de")]
    German,
    #[sea_orm(iden = "ko")]
    Korean,
    #[sea_orm(iden = "vi")]
    Vietnamese,
    #[sea_orm(iden = "tr")]
    Turkish,
    #[sea_orm(iden = "it")]
    Italian,
    #[sea_orm(iden = "th")]
    Thai,
    #[sea_orm(iden = "pl")]
    Polish,
    #[sea_orm(iden = "nl")]
    Dutch,
}

#[derive(DeriveIden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ListenEvent {
    #[sea_orm(iden = "init_clipboards")]
    InitClipboards,
    #[sea_orm(iden = "init_hotkeys")]
    InitHotkeys,
    #[sea_orm(iden = "init_settings")]
    InitSettings,
    #[sea_orm(iden = "enable_global_hotkey_event")]
    EnableGlobalHotkeyEvent,
    #[sea_orm(iden = "change_tab")]
    ChangeTab,
    #[sea_orm(iden = "scroll_to_top")]
    ScrollToTop,
    #[sea_orm(iden = "new_clipboard")]
    NewClipboard,
    #[sea_orm(iden = "progress")]
    Progress,
    #[sea_orm(iden = "password_lock")]
    PasswordLock,
}

#[derive(DeriveIden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HotkeyEvent {
    #[sea_orm(iden = "window_display_toggle")]
    WindowDisplayToggle,
    #[sea_orm(iden = "type_clipboard")]
    TypeClipboard,
    #[sea_orm(iden = "scroll_to_top")]
    ScrollToTop,
    #[sea_orm(iden = "sync_clipboard_history")]
    SyncClipboardHistory,
    #[sea_orm(iden = "settings")]
    Settings,
    #[sea_orm(iden = "about")]
    About,
    #[sea_orm(iden = "exit")]
    Exit,
    #[sea_orm(iden = "recent_clipboards")]
    RecentClipboards,
    #[sea_orm(iden = "starred_clipboards")]
    StarredClipboards,
    #[sea_orm(iden = "history")]
    History,
    #[sea_orm(iden = "view_more")]
    ViewMore,
    #[sea_orm(iden = "digit_1")]
    Digit1,
    #[sea_orm(iden = "digit_2")]
    Digit2,
    #[sea_orm(iden = "digit_3")]
    Digit3,
    #[sea_orm(iden = "digit_4")]
    Digit4,
    #[sea_orm(iden = "digit_5")]
    Digit5,
    #[sea_orm(iden = "digit_6")]
    Digit6,
    #[sea_orm(iden = "digit_7")]
    Digit7,
    #[sea_orm(iden = "digit_8")]
    Digit8,
    #[sea_orm(iden = "digit_9")]
    Digit9,
    #[sea_orm(iden = "num_1")]
    Num1,
    #[sea_orm(iden = "num_2")]
    Num2,
    #[sea_orm(iden = "num_3")]
    Num3,
    #[sea_orm(iden = "num_4")]
    Num4,
    #[sea_orm(iden = "num_5")]
    Num5,
    #[sea_orm(iden = "num_6")]
    Num6,
    #[sea_orm(iden = "num_7")]
    Num7,
    #[sea_orm(iden = "num_8")]
    Num8,
    #[sea_orm(iden = "num_9")]
    Num9,
}

#[derive(DeriveIden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WebWindow {
    #[sea_orm(iden = "main")]
    Main,
    #[sea_orm(iden = "about")]
    About,
    #[sea_orm(iden = "settings")]
    Settings,
}

#[derive(DeriveIden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ClipboardTextType {
    #[sea_orm(iden = "text")]
    Text,
    #[sea_orm(iden = "link")]
    Link,
    #[sea_orm(iden = "hex")]
    Hex,
    #[sea_orm(iden = "rgb")]
    Rgb,
}

#[derive(DeriveIden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ClipboardType {
    #[sea_orm(iden = "text")]
    Text,
    #[sea_orm(iden = "image")]
    Image,
    #[sea_orm(iden = "html")]
    Html,
    #[sea_orm(iden = "rtf")]
    Rtf,
    #[sea_orm(iden = "file")]
    File,
}

#[derive(DeriveIden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PasswordAction {
    #[sea_orm(iden = "encrypt")]
    Encrypt,
    #[sea_orm(iden = "decrypt")]
    Decrypt,
    #[sea_orm(iden = "sync_decrypt")]
    SyncDecrypt,
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
