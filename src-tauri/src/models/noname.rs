use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{path::BaseDirectory, AppHandle, Manager};

#[derive(Deserialize, Serialize)]
pub struct Noname {
    pub path: PathBuf,
}

impl Noname {
    pub fn new(handle: &AppHandle) -> Self {
        let path = handle
            .path()
            .resolve("noname", BaseDirectory::AppLocalData)
            .expect("Cannot access AppLocalData directory");

        if !path.exists() {
            fs::create_dir_all(path.clone()).expect("Cannot create noname path");
        }

        Self { path }
    }
}
