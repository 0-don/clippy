// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_variables)]

use entity::post;

use sea_orm::{ActiveModelTrait, Set};
use sea_orm::{DatabaseConnection, DbErr};

mod connection;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(name: &str) -> Result<String, String> {
    let res = insert().await;
    Ok(format!(
        "Hello, {}! You've been greeted from Rust!",
        res.unwrap()
    ))
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

    Ok(format!(
        "Post created with ID: {}, TITLE: {}",
        post.id, post.title,
    ))
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
