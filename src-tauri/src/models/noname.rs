use std::{fs, path::PathBuf, time::UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::utils::fs::noname_path;

#[derive(Deserialize, Serialize)]
pub struct NonameStatus {
    pub path: PathBuf,
    pub updated_at: Option<u128>,
}

impl NonameStatus {
    pub fn new(app: &AppHandle) -> Self {
        let path = noname_path(app);

        if !path.exists() {
            fs::create_dir_all(path.clone()).expect("Cannot create noname path");
        }

        let updated_at = path
            .join("index.html")
            .metadata()
            .ok()
            .and_then(|meta| meta.modified().ok())
            .and_then(|updated_at| updated_at.duration_since(UNIX_EPOCH).ok())
            .map(|updated_at| updated_at.as_millis());

        Self { path, updated_at }
    }
}
