use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::utils::fs::noname_path;

#[derive(Deserialize, Serialize)]
pub struct NonameStatus {
    pub path: PathBuf,
}

impl NonameStatus {
    pub fn new(app: &AppHandle) -> Self {
        let path = noname_path(app);

        if !path.exists() {
            fs::create_dir_all(path.clone()).expect("Cannot create noname path");
        }

        Self { path }
    }
}
