pub mod clipboard_manager;
pub mod fullscreen_detector;
pub mod hotkey_manager;
#[cfg(not(all(target_os = "windows", target_arch = "aarch64")))]
pub mod ocr;
pub mod providers;
pub mod sync_manager;
