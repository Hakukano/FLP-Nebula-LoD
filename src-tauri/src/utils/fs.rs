use std::path::PathBuf;

use tauri::{path::BaseDirectory, AppHandle, Manager};

pub fn noname_path(app: &AppHandle) -> PathBuf {
    app.path()
        .resolve("noname", BaseDirectory::AppLocalData)
        .expect("Cannot access AppLocalData directory")
}
