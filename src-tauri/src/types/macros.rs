#[macro_export]
macro_rules! define_hotkey_event {
    ($($name:ident => $value:expr),* $(,)?) => {
        #[derive(Debug)]
        pub enum HotkeyEvent {
            $($name,)*
        }

        impl std::str::FromStr for HotkeyEvent {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($value => Ok(HotkeyEvent::$name),)*
                    _ => Err(()),
                }
            }
        }
    };
}
