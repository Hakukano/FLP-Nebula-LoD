use regex::Regex;
use serde::Serialize;
use tauri::{AppHandle, Manager};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};

use crate::{models::noname::NonameStatus, utils::fs::noname_path, AppState};

#[derive(Debug, thiserror::Error, Serialize)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Shell command error: {0}")]
    TauriPluginShell(#[from] tauri_plugin_shell::Error),
    #[error("Gix create error: {0}")]
    GixCreate(String),
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
pub fn noname_update(app: AppHandle, repo: String, branch: String) -> Result<(), Error> {
    let path = noname_path(app.app_handle());
    std::fs::remove_dir_all(path.as_path()).map_err(|err| Error::Io(err.to_string()))?;

    let mut prepare_fetch = gix::prepare_clone(repo, path)
        .map_err(|err| Error::GixCreate(err.to_string()))?
        .with_ref_name(Some(branch.as_str()))
        .map_err(|err| Error::GixCreate(err.to_string()))?;

    let (mut prepare_checkout, _) = prepare_fetch
        .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
        .map_err(|err| Error::GixCreate(err.to_string()))?;
    println!(
        "Checking out into {:?} ...",
        prepare_checkout.repo().work_dir().expect("should be there")
    );

    let (repo, _) = prepare_checkout
        .main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
        .map_err(|err| Error::GixCreate(err.to_string()))?;
    println!(
        "Repo cloned into {:?}",
        repo.work_dir().expect("directory pre-created")
    );

    Ok(())
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
            "--parent-pid",
            std::process::id().to_string().as_str(),
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
