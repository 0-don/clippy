/// Emits through the `log` crate so tauri-plugin-log routes it to stdout AND a file
/// (the file target works even in the Windows release build, which has no console).
#[macro_export]
macro_rules! printlog {
    ($($arg:tt)*) => {
        log::info!($($arg)*);
    };
}
