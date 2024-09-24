use std::path::PathBuf;

use tauri::{path::BaseDirectory, AppHandle, Manager};

pub fn noname_path(handle: &AppHandle) -> PathBuf {
    handle
        .path()
        .resolve("noname", BaseDirectory::AppLocalData)
        .expect("Cannot access AppLocalData directory")
}
