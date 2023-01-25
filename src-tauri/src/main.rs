#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod status;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn sha3(value: &str) -> String {
    match status::sha3(value) {
        Ok(value) => return value,
        Err(err) => {eprintln!("Error :( \n  {}", err);
    return String::from("error")},
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, sha3])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
