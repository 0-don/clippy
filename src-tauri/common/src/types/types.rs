use global_hotkey::hotkey::HotKey;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub db: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Progress {
    pub label: String,
    pub total: usize,
    pub current: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPath {
    pub config_path: String,
    pub db_file_path: String,
    pub config_file_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatabaseInfo {
    pub records: u64,
    pub size: u64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Key {
    pub id: u32,
    pub state: bool,
    pub is_global: bool,
    pub key_str: String,
    pub event: String,
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub key: String,
    pub hotkey: HotKey,
}

#[derive(Debug)]
pub enum KeyboardLayout {
    Qwerty,
    Qwertz,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandError {
    Error(String),
}

impl CommandError {
    pub fn new(msg: &str) -> Self {
        CommandError::Error(msg.to_string())
    }
}

// Simplified error handling using a single variant
impl<E: std::error::Error> From<E> for CommandError {
    fn from(err: E) -> Self {
        CommandError::Error(err.to_string())
    }
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone)]
pub struct TextMatcher {
    match_expression: String, // or search_expression
    substitution: String,     // or replacement_text
    enabled: bool,
}

impl TextMatcher {
    pub fn replace_matches(&self, text: &str) -> Option<String> {
        if !self.enabled || self.match_expression.is_empty() {
            return None;
        }

        // Try as glob pattern first
        if let Ok(glob) = globset::GlobBuilder::new(&self.match_expression)
            .case_insensitive(true)
            .build()
        {
            let words: Vec<&str> = text.split_whitespace().collect();
            let mut result = text.to_string();
            let mut changed = false;

            for word in words {
                if glob.compile_matcher().is_match(word) {
                    result = result.replace(word, &self.substitution);
                    changed = true;
                }
            }

            if changed {
                return Some(result);
            }
        }

        // Try as Regex if glob doesn't match
        if let Ok(regex) = regex::RegexBuilder::new(&self.match_expression)
            .case_insensitive(true)
            .build()
        {
            let replaced = regex.replace_all(text, &self.substitution);
            if replaced != text {
                return Some(replaced.into_owned());
            }
        }

        None
    }

    pub fn from_json_value(value: &JsonValue) -> Vec<Self> {
        match value {
            JsonValue::Array(arr) => serde_json::from_value(json!(arr)).unwrap_or_else(|_| vec![]),
            _ => vec![],
        }
    }
}
