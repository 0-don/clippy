use std::collections::HashMap;

/// Physical keys whose character depends on the layout, as
/// (KeyboardEvent.code, X11 keycode, Windows scancode).
const LAYOUT_KEYS: &[(&str, u8, u32)] = &[
    ("Backquote", 49, 0x29),
    ("Digit1", 10, 0x02),
    ("Digit2", 11, 0x03),
    ("Digit3", 12, 0x04),
    ("Digit4", 13, 0x05),
    ("Digit5", 14, 0x06),
    ("Digit6", 15, 0x07),
    ("Digit7", 16, 0x08),
    ("Digit8", 17, 0x09),
    ("Digit9", 18, 0x0A),
    ("Digit0", 19, 0x0B),
    ("Minus", 20, 0x0C),
    ("Equal", 21, 0x0D),
    ("KeyQ", 24, 0x10),
    ("KeyW", 25, 0x11),
    ("KeyE", 26, 0x12),
    ("KeyR", 27, 0x13),
    ("KeyT", 28, 0x14),
    ("KeyY", 29, 0x15),
    ("KeyU", 30, 0x16),
    ("KeyI", 31, 0x17),
    ("KeyO", 32, 0x18),
    ("KeyP", 33, 0x19),
    ("BracketLeft", 34, 0x1A),
    ("BracketRight", 35, 0x1B),
    ("KeyA", 38, 0x1E),
    ("KeyS", 39, 0x1F),
    ("KeyD", 40, 0x20),
    ("KeyF", 41, 0x21),
    ("KeyG", 42, 0x22),
    ("KeyH", 43, 0x23),
    ("KeyJ", 44, 0x24),
    ("KeyK", 45, 0x25),
    ("KeyL", 46, 0x26),
    ("Semicolon", 47, 0x27),
    ("Quote", 48, 0x28),
    ("Backslash", 51, 0x2B),
    ("IntlBackslash", 94, 0x56),
    ("KeyZ", 52, 0x2C),
    ("KeyX", 53, 0x2D),
    ("KeyC", 54, 0x2E),
    ("KeyV", 55, 0x2F),
    ("KeyB", 56, 0x30),
    ("KeyN", 57, 0x31),
    ("KeyM", 58, 0x32),
    ("Comma", 59, 0x33),
    ("Period", 60, 0x34),
    ("Slash", 61, 0x35),
];

/// Maps each physical key to the character it types unshifted on the active
/// layout, like Chromium's `navigator.keyboard.getLayoutMap()`, which the
/// WebKit webviews lack. Empty where the layout cannot be read, and on macOS,
/// where hotkeys bind physical keys anyway.
pub fn get_keyboard_layout_map() -> HashMap<String, String> {
    #[cfg(target_os = "linux")]
    return linux::layout_map().unwrap_or_default();
    #[cfg(target_os = "windows")]
    return windows::layout_map();
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    return HashMap::new();
}

#[cfg(target_os = "linux")]
mod linux {
    use super::LAYOUT_KEYS;
    use std::collections::HashMap;
    use x11rb::connection::Connection;
    use x11rb::protocol::xproto::ConnectionExt;

    pub fn layout_map() -> Option<HashMap<String, String>> {
        let (conn, _) = x11rb::connect(None).ok()?;
        let min = conn.setup().min_keycode;
        let max = conn.setup().max_keycode;
        let mapping = conn
            .get_keyboard_mapping(min, max - min + 1)
            .ok()?
            .reply()
            .ok()?;
        let per_keycode = usize::from(mapping.keysyms_per_keycode);

        Some(
            LAYOUT_KEYS
                .iter()
                .filter_map(|&(code, keycode, _)| {
                    let index = usize::from(keycode.checked_sub(min)?) * per_keycode;
                    let keysym = *mapping.keysyms.get(index)?;
                    Some((code.to_string(), keysym_to_char(keysym)?.to_string()))
                })
                .collect(),
        )
    }

    /// Latin-1 keysyms equal their code point; Unicode keysyms carry it with a
    /// 0x01000000 offset. Legacy non Latin keysyms stay unmapped.
    fn keysym_to_char(keysym: u32) -> Option<char> {
        match keysym {
            0x20..=0x7E | 0xA0..=0xFF => char::from_u32(keysym),
            0x0100_0100..=0x0110_FFFF => char::from_u32(keysym - 0x0100_0000),
            _ => None,
        }
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use super::LAYOUT_KEYS;
    use std::collections::HashMap;
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
        GetKeyboardLayout, MapVirtualKeyExW, ToUnicodeEx, MAPVK_VSC_TO_VK_EX,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowThreadProcessId,
    };

    /// Flag 4 keeps ToUnicodeEx from consuming a pending dead key.
    const DONT_CHANGE_KERNEL_STATE: u32 = 4;

    pub fn layout_map() -> HashMap<String, String> {
        // The layout is per thread; the focused window's is the one being typed with.
        let layout = unsafe {
            let thread = GetWindowThreadProcessId(GetForegroundWindow(), std::ptr::null_mut());
            GetKeyboardLayout(thread)
        };
        let key_state = [0u8; 256];

        LAYOUT_KEYS
            .iter()
            .filter_map(|&(code, _, scancode)| {
                let mut buffer = [0u16; 8];
                let written = unsafe {
                    let vk = MapVirtualKeyExW(scancode, MAPVK_VSC_TO_VK_EX, layout);
                    ToUnicodeEx(
                        vk,
                        scancode,
                        key_state.as_ptr(),
                        buffer.as_mut_ptr(),
                        buffer.len() as i32,
                        DONT_CHANGE_KERNEL_STATE,
                        layout,
                    )
                };
                // Negative means a dead key; its character is still in the buffer.
                let length = written.unsigned_abs() as usize;
                let text = String::from_utf16(&buffer[..length.min(buffer.len())]).ok()?;
                let char = text.chars().next().filter(|c| !c.is_control())?;
                Some((code.to_string(), char.to_string()))
            })
            .collect()
    }
}
