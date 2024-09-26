// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::sync::Mutex;

use controllers::git::GitUpdate;
use tauri::Manager;
use tauri_plugin_shell::process::CommandChild;

mod controllers;
mod models;
mod utils;

#[derive(Default)]
struct AppState {
    git_command_child: Mutex<Option<CommandChild>>,
    git_update: Mutex<Option<GitUpdate>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            controllers::git::git_launch,
            controllers::git::git_status,
            controllers::git::git_update,
            controllers::git::git_update_status,
        ])
        .setup(|app| {
            app.manage(AppState::default());
            Ok(())
        })
        .run(tauri::generate_context!());
}
