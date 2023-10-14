use entity::{hotkey, settings};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        settings::ActiveModel {
            startup: Set(true),
            dark_mode: Set(true),
            notification: Set(false),
            synchronize: Set(false),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("window_display_toggle".to_string()),
            ctrl: Set(true),
            alt: Set(false),
            shift: Set(false),
            key: Set("Y".to_string()),
            status: Set(true),
            name: Set("Clippy Display Toggle".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path fill=\\\"currentColor\\\" d=\\\"M3 17h18v2H3v-2Zm0-6h3v3H3v-3Zm5 0h3v3H8v-3ZM3 5h3v3H3V5Zm10 0h3v3h-3V5Zm5 0h3v3h-3V5Zm-5 6h3v3h-3v-3Zm5 0h3v3h-3v-3ZM8 5h3v3H8V5Z\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("scroll_to_top".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("E".to_string()),
            status: Set(true),
            name: Set("Scroll to Top".to_string()),
            icon: Set("\"<svg stroke-width=\\\"2\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" stroke=\\\"currentColor\\\" stroke-linecap=\\\"round\\\" stroke-linejoin=\\\"round\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M12 19V5M5 12l7-7 7 7\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("recent_clipboards".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("R".to_string()),
            status: Set(true),
            name: Set("Recent Clipboards".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"currentColor\\\" viewBox=\\\"0 0 16 16\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path fill-rule=\\\"evenodd\\\" d=\\\"M13.507 12.324a7 7 0 0 0 .065-8.56A7 7 0 0 0 2 4.393V2H1v3.5l.5.5H5V5H2.811a6.008 6.008 0 1 1-.135 5.77l-.887.462a7 7 0 0 0 11.718 1.092zm-3.361-.97.708-.707L8 7.792V4H7v4l.146.354 3 3z\\\" clip-rule=\\\"evenodd\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("starred_clipboards".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("T".to_string()),
            status: Set(true),
            name: Set("Starred Clipboards".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"currentColor\\\" viewBox=\\\"0 0 16 16\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M3.612 15.443c-.386.198-.824-.149-.746-.592l.83-4.73L.173 6.765c-.329-.314-.158-.888.283-.95l4.898-.696L7.538.792c.197-.39.73-.39.927 0l2.184 4.327 4.898.696c.441.062.612.636.282.95l-3.522 3.356.83 4.73c.078.443-.36.79-.746.592L8 13.187l-4.389 2.256z\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("history".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("H".to_string()),
            status: Set(true),
            name: Set("History".to_string()),
            icon: Set("\"<svg stroke-width=\\\"2\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" stroke=\\\"currentColor\\\" stroke-linecap=\\\"round\\\" stroke-linejoin=\\\"round\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path stroke=\\\"none\\\" d=\\\"M0 0h24v24H0z\\\"></path><path d=\\\"M3 10a7 7 0 1 0 14 0 7 7 0 1 0-14 0M21 21l-6-6\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("view_more".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("M".to_string()),
            status: Set(true),
            name: Set("View more".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path fill=\\\"currentColor\\\" fill-rule=\\\"evenodd\\\" d=\\\"M5 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Zm0-2a1 1 0 1 0 0-2 1 1 0 0 0 0 2ZM12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6Zm0-2a1 1 0 1 0 0-2 1 1 0 0 0 0 2ZM22 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Zm-2 0a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z\\\" clip-rule=\\\"evenodd\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("sync_clipboard_history".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("P".to_string()),
            status: Set(true),
            name: Set("Sync Clipboard History".to_string()),
            icon: Set("\"<svg stroke-width=\\\"2\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" stroke=\\\"currentColor\\\" stroke-linecap=\\\"round\\\" stroke-linejoin=\\\"round\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z\\\"></path><path d=\\\"M17 21v-8H7v8M7 3v5h8\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("preferences".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("O".to_string()),
            status: Set(true),
            name: Set("Preferences".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" stroke=\\\"currentColor\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path fill=\\\"currentColor\\\" fill-rule=\\\"evenodd\\\" d=\\\"M11.828 2.25c-.916 0-1.699.663-1.85 1.567l-.091.549a.798.798 0 0 1-.517.608 7.45 7.45 0 0 0-.478.198.798.798 0 0 1-.796-.064l-.453-.324a1.875 1.875 0 0 0-2.416.2l-.243.243a1.875 1.875 0 0 0-.2 2.416l.324.453a.798.798 0 0 1 .064.796 7.448 7.448 0 0 0-.198.478.798.798 0 0 1-.608.517l-.55.092a1.875 1.875 0 0 0-1.566 1.849v.344c0 .916.663 1.699 1.567 1.85l.549.091c.281.047.508.25.608.517.06.162.127.321.198.478a.798.798 0 0 1-.064.796l-.324.453a1.875 1.875 0 0 0 .2 2.416l.243.243c.648.648 1.67.733 2.416.2l.453-.324a.798.798 0 0 1 .796-.064c.157.071.316.137.478.198.267.1.47.327.517.608l.092.55c.15.903.932 1.566 1.849 1.566h.344c.916 0 1.699-.663 1.85-1.567l.091-.549a.798.798 0 0 1 .517-.608 7.52 7.52 0 0 0 .478-.198.798.798 0 0 1 .796.064l.453.324a1.875 1.875 0 0 0 2.416-.2l.243-.243c.648-.648.733-1.67.2-2.416l-.324-.453a.798.798 0 0 1-.064-.796c.071-.157.137-.316.198-.478.1-.267.327-.47.608-.517l.55-.091a1.875 1.875 0 0 0 1.566-1.85v-.344c0-.916-.663-1.699-1.567-1.85l-.549-.091a.798.798 0 0 1-.608-.517 7.507 7.507 0 0 0-.198-.478.798.798 0 0 1 .064-.796l.324-.453a1.875 1.875 0 0 0-.2-2.416l-.243-.243a1.875 1.875 0 0 0-2.416-.2l-.453.324a.798.798 0 0 1-.796.064 7.462 7.462 0 0 0-.478-.198.798.798 0 0 1-.517-.608l-.091-.55a1.875 1.875 0 0 0-1.85-1.566h-.344ZM12 15.75a3.75 3.75 0 1 0 0-7.5 3.75 3.75 0 0 0 0 7.5Z\\\" clip-rule=\\\"evenodd\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("about".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("I".to_string()),
            status: Set(true),
            name: Set("About".to_string()),
            icon: Set("\"<svg stroke-width=\\\"0\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path fill=\\\"currentColor\\\" d=\\\"M11 10.98a1 1 0 1 1 2 0v6a1 1 0 1 1-2 0v-6ZM12 6.051a1 1 0 1 0 0 2 1 1 0 0 0 0-2Z\\\"></path><path fill=\\\"currentColor\\\" fill-rule=\\\"evenodd\\\" d=\\\"M12 2C6.477 2 2 6.477 2 12s4.477 10 10 10 10-4.477 10-10S17.523 2 12 2ZM4 12a8 8 0 1 0 16 0 8 8 0 0 0-16 0Z\\\" clip-rule=\\\"evenodd\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("exit".to_string()),
            ctrl: Set(false),
            alt: Set(false),
            shift: Set(false),
            key: Set("X".to_string()),
            status: Set(true),
            name: Set("Exit".to_string()),
            icon: Set("\"<svg stroke-width=\\\"2\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" stroke=\\\"currentColor\\\" stroke-linecap=\\\"round\\\" stroke-linejoin=\\\"round\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4M16 17l5-5-5-5M21 12H9\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        hotkey::ActiveModel {
            event: Set("type_clipboard".to_string()),
            ctrl: Set(true),
            alt: Set(false),
            shift: Set(false),
            key: Set("B".to_string()),
            status: Set(true),
            name: Set("Type Clipboard".to_string()),
            icon: Set("\"<svg stroke-width=\\\"2\\\" height=\\\"1em\\\" width=\\\"1em\\\" xmlns=\\\"http://www.w3.org/2000/svg\\\" fill=\\\"none\\\" stroke=\\\"currentColor\\\" stroke-linecap=\\\"round\\\" stroke-linejoin=\\\"round\\\" viewBox=\\\"0 0 24 24\\\" color=\\\"currentColor\\\" style=\\\"overflow: visible;\\\"><path d=\\\"M4 7V4h16v3M9 20h6M12 4v16\\\"></path></svg>\"".to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
