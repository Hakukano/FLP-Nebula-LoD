use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::utils::fs::noname_path;

#[derive(Deserialize, Serialize)]
pub struct Noname {
    pub path: PathBuf,
}

impl Noname {
    pub fn new(handle: &AppHandle) -> Self {
        let path = noname_path(handle);

        if !path.exists() {
            fs::create_dir_all(path.clone()).expect("Cannot create noname path");
        }

        Self { path }
    }
}
