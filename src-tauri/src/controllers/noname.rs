use regex::Regex;
use serde::Serialize;
use tauri::{AppHandle, Manager};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};

use crate::{models::noname::NonameStatus, utils::fs::noname_path, AppState};

#[derive(Debug, thiserror::Error, Serialize)]
pub enum Error {
    #[error("Shell command error: {0}")]
    TauriPluginShell(#[from] tauri_plugin_shell::Error),
    #[error("Shell command is already running, please restart the app")]
    ShellCommandAlreadyRunning,
    #[error("Shell command failed")]
    ShellCommandFailure(Vec<String>),
}

#[tauri::command]
pub fn noname_status(app: AppHandle) -> NonameStatus {
    NonameStatus::new(&app)
}

#[tauri::command]
pub async fn noname_launch(app: AppHandle, expose: bool) -> Result<String, Error> {
    if app
        .state::<AppState>()
        .noname_command_child
        .lock()
        .expect("Cannot acquire lock of app state")
        .is_some()
    {
        return Err(Error::ShellCommandAlreadyRunning);
    }

    let noname_command = app
        .shell()
        .sidecar("noname")
        .expect("noname server not found")
        .args([
            "--bind-ip",
            if expose { "0.0.0.0:0" } else { "127.0.0.1:0" },
            "--base-path",
            noname_path(app.app_handle())
                .to_str()
                .expect("Invalid noname base path"),
        ]);
    let (mut rx, child) = noname_command.spawn()?;

    let port_regex = Regex::new(r"Listening on \d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:(\d+)")
        .expect("Invalid regex");

    let mut error = Vec::new();
    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(line) => {
                if let Ok(line) = String::from_utf8(line) {
                    if let Some(captures) = port_regex.captures(line.as_str()) {
                        if let Some(port) = captures.get(1) {
                            app.state::<AppState>()
                                .noname_command_child
                                .lock()
                                .expect("Cannot acquire lock of app state")
                                .replace(child);
                            return Ok(format!("http://localhost:{}/index.html", port.as_str()));
                        }
                    }
                }
            }
            CommandEvent::Stderr(line) => error.push(String::from_utf8(line).unwrap_or_default()),
            CommandEvent::Error(err) => error.push(err),
            _ => break,
        }
    }

    Err(Error::ShellCommandFailure(error))
}
