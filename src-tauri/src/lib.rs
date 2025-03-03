use std::sync::Arc;

use tokio::sync::Mutex;

mod audio;
mod detc;
mod init;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(detc::AppState {
            command_sender: Mutex::new(None),
        })
        .plugin(tauri_plugin_opener::init())
        .setup(init::init)
        .invoke_handler(tauri::generate_handler![
            greet,
            detc::start_pitch_detection,
            detc::stop_pitch_detection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
