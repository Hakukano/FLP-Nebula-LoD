// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use tauri_plugin_shell::ShellExt;
use utils::fs::noname_path;

mod controllers;
mod models;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![controllers::noname::noname_index])
        .setup(move |app| {
            let noname_command = app
                .shell()
                .sidecar("noname")
                .expect("noname server not found")
                .args([noname_path(app.handle())]);
            noname_command.spawn()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
