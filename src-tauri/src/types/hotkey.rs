use global_hotkey::{hotkey::HotKey, GlobalHotKeyManager};

pub struct SafeHotKeyManager(GlobalHotKeyManager);

unsafe impl Send for SafeHotKeyManager {}
unsafe impl Sync for SafeHotKeyManager {}

impl SafeHotKeyManager {
    pub fn new(manager: GlobalHotKeyManager) -> Self {
        Self(manager)
    }

    pub fn register_all(&self, hotkeys: &[HotKey]) -> global_hotkey::Result<()> {
        self.0.register_all(hotkeys)
    }

    pub fn unregister_all(&self, hotkeys: &[HotKey]) -> global_hotkey::Result<()> {
        self.0.unregister_all(hotkeys)
    }
}