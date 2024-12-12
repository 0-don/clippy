#[derive(Debug)]
pub enum KeyboardLayout {
    Qwerty,
    Qwertz,
}

pub fn get_keyboard_layout() -> KeyboardLayout {
    if cfg!(target_os = "linux") {
        let output = std::process::Command::new("setxkbmap")
            .arg("-query")
            .output();

        if let Ok(result) = output.as_ref() {
            let output_str: Vec<String> = std::str::from_utf8(&result.stdout)
                .expect("Failed to parse setxkbmap output")
                .split("\n")
                .map(|x| x.trim().to_lowercase())
                .collect();

            if output_str
                .iter()
                .any(|x| x.contains("de") && x.contains("layout:"))
            {
                println!("Linux QWERTY");
                return KeyboardLayout::Qwerty;
            }
        }
    } else if cfg!(target_os = "windows") {
        let output = std::process::Command::new("reg")
            .arg("query")
            .arg(r"HKCU\Keyboard Layout\Preload")
            .output();

        if let Ok(result) = output.as_ref() {
            let output_str =
                std::str::from_utf8(&result.stdout).expect("Failed to parse reg output");
            if !output_str.contains("00000409") {
                println!("Windows QWERTY");
                return KeyboardLayout::Qwerty;
            }
        }
    } else if cfg!(target_os = "macos") {
        let output = std::process::Command::new("defaults")
            .arg("read")
            .arg("-g")
            .arg("AppleCurrentKeyboardLayoutInputSourceID")
            .output();

        if let Ok(result) = output.as_ref() {
            let output_str =
                std::str::from_utf8(&result.stdout).expect("Failed to parse defaults output");
            if !output_str.contains("com.apple.keylayout.US") {
                println!("Mac QWERTY");
                return KeyboardLayout::Qwerty;
            }
        }
    }
    // Default to QWERTZ for unsupported OS
    println!("{} default QWERTZ", std::env::consts::OS);
    KeyboardLayout::Qwertz
}
