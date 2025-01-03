use sea_orm::prelude::*;
use sea_orm::{sea_query, EnumIter};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

#[derive(Iden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ClippyPosition {
    #[iden = "cursor"]
    Cursor,
    #[iden = "top_left"]
    TopLeft,
    #[iden = "top_right"]
    TopRight,
    #[iden = "bottom_left"]
    BottomLeft,
    #[iden = "bottom_right"]
    BottomRight,
    #[iden = "top_center"]
    TopCenter,
    #[iden = "bottom_center"]
    BottomCenter,
    #[iden = "left_center"]
    LeftCenter,
    #[iden = "right_center"]
    RightCenter,
    #[iden = "center"]
    Center,
    #[iden = "tray_left"]
    TrayLeft,
    #[iden = "tray_bottom_left"]
    TrayBottomLeft,
    #[iden = "tray_right"]
    TrayRight,
    #[iden = "tray_bottom_right"]
    TrayBottomRight,
    #[iden = "tray_center"]
    TrayCenter,
    #[iden = "tray_bottom_center"]
    TrayBottomCenter,
}

#[derive(Iden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FolderLocation {
    #[iden = "database"]
    Database,
    #[iden = "config"]
    Config,
}

#[derive(Iden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[iden = "en"]
    English,
    #[iden = "zh"]
    Mandarin,
    #[iden = "hi"]
    Hindi,
    #[iden = "es"]
    Spanish,
    #[iden = "fr"]
    French,
    #[iden = "ar"]
    Arabic,
    #[iden = "bn"]
    Bengali,
    #[iden = "pt"]
    Portuguese,
    #[iden = "ru"]
    Russian,
    #[iden = "ur"]
    Urdu,
    #[iden = "ja"]
    Japanese,
    #[iden = "de"]
    German,
    #[iden = "ko"]
    Korean,
    #[iden = "vi"]
    Vietnamese,
    #[iden = "tr"]
    Turkish,
    #[iden = "it"]
    Italian,
    #[iden = "th"]
    Thai,
    #[iden = "pl"]
    Polish,
    #[iden = "nl"]
    Dutch,
}

#[derive(Iden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ListenEvent {
    #[iden = "init"]
    Init,
    #[iden = "enable_global_hotkey_event"]
    EnableGlobalHotkeyEvent,
    #[iden = "change_tab"]
    ChangeTab,
    #[iden = "scroll_to_top"]
    ScrollToTop,
    #[iden = "new_clipboard"]
    NewClipboard,
}

#[derive(Iden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum HotkeyEvent {
    #[iden = "window_display_toggle"]
    WindowDisplayToggle,
    #[iden = "type_clipboard"]
    TypeClipboard,
    #[iden = "scroll_to_top"]
    ScrollToTop,
    #[iden = "sync_clipboard_history"]
    SyncClipboardHistory,
    #[iden = "settings"]
    Settings,
    #[iden = "about"]
    About,
    #[iden = "exit"]
    Exit,
    #[iden = "recent_clipboards"]
    RecentClipboards,
    #[iden = "starred_clipboards"]
    StarredClipboards,
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

#[derive(Iden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WebWindow {
    #[iden = "main"]
    Main,
    #[iden = "about"]
    About,
    #[iden = "settings"]
    Settings,
}

#[derive(Iden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
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

#[derive(Iden, EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
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
