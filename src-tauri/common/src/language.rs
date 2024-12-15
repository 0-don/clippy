use crate::types::enums::Language;
pub use sea_orm::Iden;

pub fn get_system_language() -> Language {
    if cfg!(target_os = "linux") {
        if let Ok(lang) = std::env::var("LANG") {
            let lang = lang.to_lowercase();
            match &lang[..2] {
                s if s == Language::German.to_string() => Language::German,
                s if s == Language::Spanish.to_string() => Language::Spanish,
                s if s == Language::French.to_string() => Language::French,
                _ => Language::English,
            }
        } else {
            Language::English
        }
    } else if cfg!(target_os = "windows") {
        let output = std::process::Command::new("powershell")
            .arg("-Command")
            .arg("(Get-Culture).TwoLetterISOLanguageName")
            .output();

        if let Ok(result) = output {
            if let Ok(lang) = std::str::from_utf8(&result.stdout) {
                let lang = lang.trim().to_lowercase();
                match lang.as_str() {
                    s if s == Language::German.to_string() => Language::German,
                    s if s == Language::Spanish.to_string() => Language::Spanish,
                    s if s == Language::French.to_string() => Language::French,
                    _ => Language::English,
                }
            } else {
                Language::English
            }
        } else {
            Language::English
        }
    } else if cfg!(target_os = "macos") {
        let output = std::process::Command::new("defaults")
            .arg("read")
            .arg(".GlobalPreferences")
            .arg("AppleLanguages")
            .output();

        if let Ok(result) = output {
            if let Ok(lang) = std::str::from_utf8(&result.stdout) {
                let lang = lang.to_lowercase();
                match &lang[..2] {
                    s if s == Language::German.to_string() => Language::German,
                    s if s == Language::Spanish.to_string() => Language::Spanish,
                    s if s == Language::French.to_string() => Language::French,
                    _ => Language::English,
                }
            } else {
                Language::English
            }
        } else {
            Language::English
        }
    } else {
        println!("{} default English", std::env::consts::OS);
        Language::English
    }
}
