#[macro_export]
macro_rules! define_hotkey_event {
    ($($name:ident => $value:expr),* $(,)?) => {
        #[derive(Debug)]
        pub enum HotkeyEvent {
            $($name,)*
        }

        impl HotkeyEvent {
            pub fn as_str(&self) -> &'static str {
                match *self {
                    $(HotkeyEvent::$name => $value,)*
                }
            }
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
