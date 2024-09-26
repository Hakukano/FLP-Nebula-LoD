use std::{fs, path::PathBuf, time::UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::utils::fs::git_path;

#[derive(Deserialize, Serialize)]
pub struct GitStatus {
    pub name: String,
    pub path: PathBuf,
    pub updated_at: Option<u128>,
}

impl GitStatus {
    pub fn new(name: String, app: &AppHandle) -> Self {
        let path = git_path(name.as_str(), app);

        if !path.exists() {
            fs::create_dir_all(path.clone()).expect("Cannot create git path");
        }

        let updated_at = path
            .join("index.html")
            .metadata()
            .ok()
            .and_then(|meta| meta.modified().ok())
            .and_then(|updated_at| updated_at.duration_since(UNIX_EPOCH).ok())
            .map(|updated_at| updated_at.as_millis());

        Self {
            name,
            path,
            updated_at,
        }
    }
}
