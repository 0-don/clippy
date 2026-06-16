use common::io::keyboard::get_keyboard_layout;
use common::types::enums::HotkeyEvent;
use common::types::types::KeyboardLayout;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{ConnectionTrait, Statement, Value};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = manager.get_database_backend();

        // Use raw SQL throughout this migration to avoid depending on the current
        // entity definitions, which may include columns (e.g. hotkey.super_key,
        // added in a later migration) that don't exist yet at this migration step.
        // Inserting via `entity::hotkey::ActiveModel` here would emit SQL for the
        // full, current schema and fail on a fresh database.
        db.execute(Statement::from_string(
            backend,
            "INSERT INTO settings DEFAULT VALUES".to_owned(),
        ))
        .await?;

        let toggle_key = match get_keyboard_layout() {
            KeyboardLayout::Qwerty => "Y",
            KeyboardLayout::Qwertz => "D",
        };

        // (event, ctrl, key, name, icon) for each default hotkey. Only the columns
        // that exist at migration step 9 are written; the rest take table defaults.
        let hotkeys: [(String, bool, &str, &str, &str); 11] = [
            (HotkeyEvent::WindowDisplayToggle.to_string(), true, toggle_key, "MAIN.HOTKEY.WINDOW_DISPLAY_TOGGLE", "\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path fill=\\\"currentColor\\\" d=\\\"M3 17h18v2H3v-2Zm0-6h3v3H3v-3Zm5 0h3v3H8v-3ZM3 5h3v3H3V5Zm10 0h3v3h-3V5Zm5 0h3v3h-3V5Zm-5 6h3v3h-3v-3Zm5 0h3v3h-3v-3ZM8 5h3v3H8V5Z\\\"></path></svg>\""),
            (HotkeyEvent::ScrollToTop.to_string(), false, "E", "MAIN.HOTKEY.SCROLL_TO_TOP", "\"<svg stroke-width=\\\"2\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" stroke=\\\"currentColor\\\" stroke-linecap=\\\"round\\\" stroke-linejoin=\\\"round\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M12 19V5M5 12l7-7 7 7\\\"></path></svg>\""),
            (HotkeyEvent::RecentClipboards.to_string(), false, "R", "MAIN.HOTKEY.RECENT_CLIPBOARDS", "\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"currentColor\\\" viewBox=\\\"0 0 16 16\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path fill-rule=\\\"evenodd\\\" d=\\\"M13.507 12.324a7 7 0 0 0 .065-8.56A7 7 0 0 0 2 4.393V2H1v3.5l.5.5H5V5H2.811a6.008 6.008 0 1 1-.135 5.77l-.887.462a7 7 0 0 0 11.718 1.092zm-3.361-.97.708-.707L8 7.792V4H7v4l.146.354 3 3z\\\" clip-rule=\\\"evenodd\\\"></path></svg>\""),
            (HotkeyEvent::StarredClipboards.to_string(), false, "T", "MAIN.HOTKEY.STARRED_CLIPBOARDS", "\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"currentColor\\\" viewBox=\\\"0 0 16 16\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M3.612 15.443c-.386.198-.824-.149-.746-.592l.83-4.73L.173 6.765c-.329-.314-.158-.888.283-.95l4.898-.696L7.538.792c.197-.39.73-.39.927 0l2.184 4.327 4.898.696c.441.062.612.636.282.95l-3.522 3.356.83 4.73c.078.443-.36.79-.746.592L8 13.187l-4.389 2.256z\\\"></path></svg>\""),
            (HotkeyEvent::History.to_string(), false, "H", "MAIN.HOTKEY.HISTORY", "\"<svg stroke-width=\\\"2\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" stroke=\\\"currentColor\\\" stroke-linecap=\\\"round\\\" stroke-linejoin=\\\"round\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path stroke=\\\"none\\\" d=\\\"M0 0h24v24H0z\\\"></path><path d=\\\"M3 10a7 7 0 1 0 14 0 7 7 0 1 0-14 0M21 21l-6-6\\\"></path></svg>\""),
            (HotkeyEvent::ViewMore.to_string(), false, "M", "MAIN.HOTKEY.VIEW_MORE", "\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path fill=\\\"currentColor\\\" fill-rule=\\\"evenodd\\\" d=\\\"M5 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Zm0-2a1 1 0 1 0 0-2 1 1 0 0 0 0 2ZM12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Zm0-2a1 1 0 1 0 0-2 1 1 0 0 0 0 2ZM22 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Zm-2 0a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z\\\" clip-rule=\\\"evenodd\\\"></path></svg>\""),
            (HotkeyEvent::SyncClipboardHistory.to_string(), false, "P", "MAIN.HOTKEY.SYNC_CLIPBOARD_HISTORY", "\"<svg stroke=\\\"currentColor\\\" fill=\\\"currentColor\\\" stroke-width=\\\"0\\\" viewBox=\\\"0 0 32 32\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M 11.4375 5 L 11.15625 5.46875 L 3.15625 18.46875 L 2.84375 18.96875 L 3.125 19.5 L 7.125 26.5 L 7.40625 27 L 24.59375 27 L 24.875 26.5 L 28.875 19.5 L 29.15625 18.96875 L 28.84375 18.46875 L 20.84375 5.46875 L 20.5625 5 Z M 13.78125 7 L 19.4375 7 L 26.21875 18 L 20.5625 18 Z M 12 7.90625 L 14.96875 12.75 L 8.03125 24.03125 L 5.15625 19 Z M 16.15625 14.65625 L 18.21875 18 L 14.09375 18 Z M 12.875 20 L 26.28125 20 L 23.40625 25 L 9.78125 25 Z\\\"></path></svg>\""),
            (HotkeyEvent::Settings.to_string(), false, "O", "MAIN.HOTKEY.SETTINGS", "\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" stroke=\\\"currentColor\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path fill=\\\"currentColor\\\" fill-rule=\\\"evenodd\\\" d=\\\"M11.828 2.25c-.916 0-1.699.663-1.85 1.567l-.091.549a.798.798 0 0 1-.517.608 7.45 7.45 0 0 0-.478.198.798.798 0 0 1-.796-.064l-.453-.324a1.875 1.875 0 0 0-2.416.2l-.243.243a1.875 1.875 0 0 0-.2 2.416l.324.453a.798.798 0 0 1 .064.796 7.448 7.448 0 0 0-.198.478.798.798 0 0 1-.608.517l-.55.092a1.875 1.875 0 0 0-1.566 1.849v.344c0 .916.663 1.699 1.567 1.85l.549.091c.281.047.508.25.608.517.06.162.127.321.198.478a.798.798 0 0 1-.064.796l-.324.453a1.875 1.875 0 0 0 .2 2.416l.243.243c.648.648 1.67.733 2.416.2l.453-.324a.798.798 0 0 1 .796-.064c.157.071.316.137.478.198.267.1.47.327.517.608l.092.55c.15.903.932 1.566 1.849 1.566h.344c.916 0 1.699-.663 1.85-1.567l.091-.549a.798.798 0 0 1 .517-.608 7.52 7.52 0 0 0 .478-.198.798.798 0 0 1 .796.064l.453.324a1.875 1.875 0 0 0 2.416-.2l.243-.243c.648-.648.733-1.67.2-2.416l-.324-.453a.798.798 0 0 1-.064-.796c.071-.157.137-.316.198-.478.1-.267.327-.47.608-.517l.55-.091a1.875 1.875 0 0 0 1.566-1.85v-.344c0-.916-.663-1.699-1.567-1.85l-.549-.091a.798.798 0 0 1-.608-.517 7.507 7.507 0 0 0-.198-.478.798.798 0 0 1 .064-.796l.324-.453a1.875 1.875 0 0 0-.2-2.416l-.243-.243a1.875 1.875 0 0 0-2.416-.2l-.453.324a.798.798 0 0 1-.796.064 7.462 7.462 0 0 0-.478-.198.798.798 0 0 1-.517-.608l-.091-.55a1.875 1.875 0 0 0-1.85-1.566h-.344ZM12 15.75a3.75 3.75 0 1 0 0-7.5 3.75 3.75 0 0 0 0 7.5Z\\\" clip-rule=\\\"evenodd\\\"></path></svg>\""),
            (HotkeyEvent::About.to_string(), false, "I", "MAIN.HOTKEY.ABOUT", "\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path fill=\\\"currentColor\\\" d=\\\"M11 10.98a1 1 0 1 1 2 0v6a1 1 0 1 1-2 0v-6ZM12 6.051a1 1 0 1 0 0 2 1 1 0 0 0 0-2Z\\\"></path><path fill=\\\"currentColor\\\" fill-rule=\\\"evenodd\\\" d=\\\"M12 2C6.477 2 2 6.477 2 12s4.477 10 10 10 10-4.477 10-10S17.523 2 12 2ZM4 12a8 8 0 1 0 16 0 8 8 0 0 0-16 0Z\\\" clip-rule=\\\"evenodd\\\"></path></svg>\""),
            (HotkeyEvent::Exit.to_string(), false, "X", "MAIN.HOTKEY.EXIT", "\"<svg stroke-width=\\\"2\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" stroke=\\\"currentColor\\\" stroke-linecap=\\\"round\\\" stroke-linejoin=\\\"round\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4M16 17l5-5-5-5M21 12H9\\\"></path></svg>\""),
            (HotkeyEvent::TypeClipboard.to_string(), true, "B", "MAIN.HOTKEY.TYPE_CLIPBOARD", "\"<svg stroke-width=\\\"2\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" stroke=\\\"currentColor\\\" stroke-linecap=\\\"round\\\" stroke-linejoin=\\\"round\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M4 7V4h16v3M9 20h6M12 4v16\\\"></path></svg>\""),
        ];

        for (event, ctrl, key, name, icon) in hotkeys {
            db.execute(Statement::from_sql_and_values(
                backend,
                "INSERT INTO hotkey (event, ctrl, alt, shift, key, status, name, icon) \
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                [
                    Value::from(event),
                    Value::from(ctrl),
                    Value::from(false), // alt
                    Value::from(false), // shift
                    Value::from(key.to_string()),
                    Value::from(true), // status
                    Value::from(name.to_string()),
                    Value::from(icon.to_string()),
                ],
            ))
            .await?;
        }

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
