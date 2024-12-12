pub use sea_orm_migration::prelude::*;

mod m000001_create_clipboard;
mod m000002_create_clipboard_text;
mod m000003_create_clipboard_image;
mod m000004_create_clipboard_html;
mod m000005_create_clipboard_rtf;
mod m000006_create_clipboard_file;
mod m000007_create_settings;
mod m000008_create_hotkey;
mod m000009_seed;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m000001_create_clipboard::Migration),
            Box::new(m000002_create_clipboard_text::Migration),
            Box::new(m000003_create_clipboard_image::Migration),
            Box::new(m000004_create_clipboard_html::Migration),
            Box::new(m000005_create_clipboard_rtf::Migration),
            Box::new(m000006_create_clipboard_file::Migration),
            Box::new(m000007_create_settings::Migration),
            Box::new(m000008_create_hotkey::Migration),
            Box::new(m000009_seed::Migration),
        ]
    }
}
