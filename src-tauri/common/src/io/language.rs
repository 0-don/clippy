use crate::types::enums::Language;
pub use sea_orm::Iden;

impl Language {
    pub fn from_iso_code(lang_code: &str) -> Self {
        match &*lang_code {
            code if code == Language::Mandarin.to_string() => Language::Mandarin,
            code if code == Language::Hindi.to_string() => Language::Hindi,
            code if code == Language::Spanish.to_string() => Language::Spanish,
            code if code == Language::French.to_string() => Language::French,
            code if code == Language::Arabic.to_string() => Language::Arabic,
            code if code == Language::Bengali.to_string() => Language::Bengali,
            code if code == Language::Portuguese.to_string() => Language::Portuguese,
            code if code == Language::Russian.to_string() => Language::Russian,
            code if code == Language::Urdu.to_string() => Language::Urdu,
            code if code == Language::Japanese.to_string() => Language::Japanese,
            code if code == Language::German.to_string() => Language::German,
            code if code == Language::Korean.to_string() => Language::Korean,
            code if code == Language::Vietnamese.to_string() => Language::Vietnamese,
            code if code == Language::Turkish.to_string() => Language::Turkish,
            code if code == Language::Italian.to_string() => Language::Italian,
            code if code == Language::Thai.to_string() => Language::Thai,
            code if code == Language::Polish.to_string() => Language::Polish,
            code if code == Language::Dutch.to_string() => Language::Dutch,
            _ => Language::English,
        }
    }
}

pub fn get_system_language() -> Language {
    if cfg!(target_os = "linux") {
        std::env::var("LANG")
            .map(|lang| Language::from_iso_code(&lang.to_lowercase()[..2]))
            .unwrap_or(Language::English)
    } else if cfg!(target_os = "windows") {
        std::process::Command::new("powershell")
            .arg("-Command")
            .arg("(Get-Culture).TwoLetterISOLanguageName")
            .output()
            .ok()
            .and_then(|result| String::from_utf8(result.stdout).ok())
            .map(|lang| Language::from_iso_code(lang.trim()))
            .unwrap_or(Language::English)
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("defaults")
            .arg("read")
            .arg(".GlobalPreferences")
            .arg("AppleLanguages")
            .output()
            .ok()
            .and_then(|result| String::from_utf8(result.stdout).ok())
            .map(|lang| Language::from_iso_code(&lang.to_lowercase()[..2]))
            .unwrap_or(Language::English)
    } else {
        Language::English
    }
}
