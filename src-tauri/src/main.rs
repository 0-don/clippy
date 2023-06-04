// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use entity::post;

use sea_orm::{ActiveModelTrait, Set};
use sea_orm::{DatabaseConnection, DbErr};

mod connection;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    tokio::spawn(async move { insert().await });
    format!("Hello, {}! You've been greeted from Rust!", name)
}

async fn insert() -> Result<String, DbErr> {
    let db: DatabaseConnection = connection::establish_connection().await?;

    let post = post::ActiveModel {
        title: Set(String::from("Amazing title 1")),
        text: Set(String::from("Lorem ipsum dolor sit amet.")),
        ..Default::default()
    };

    let post: post::Model = post.insert(&db).await?;

    println!("Post created with ID: {}, TITLE: {}", post.id, post.title);

    Ok(String::from("ok"))
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
