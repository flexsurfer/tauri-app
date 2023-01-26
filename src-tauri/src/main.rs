#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use lazy_static::lazy_static;

mod backend;

lazy_static! {
    static ref API: backend::api::Api = backend::api::Api::init().unwrap();
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn sha3(value: &str) -> String {
    return API.sha3(value);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, sha3])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
