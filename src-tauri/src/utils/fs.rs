use std::path::PathBuf;

use tauri::{path::BaseDirectory, AppHandle, Manager};

pub fn git_path(name: &str, app: &AppHandle) -> PathBuf {
    app.path()
        .resolve(name, BaseDirectory::AppLocalData)
        .expect("Cannot access AppLocalData directory")
}
