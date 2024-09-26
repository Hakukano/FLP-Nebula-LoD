use std::sync::mpsc::{channel, Receiver, TryRecvError};

use regex::Regex;
use serde::Serialize;
use tauri::{async_runtime::spawn_blocking, AppHandle, Manager};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};

use crate::{models::git::GitStatus, utils::fs::git_path, AppState};

#[derive(Debug, thiserror::Error, Serialize)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Shell command error: {0}")]
    TauriPluginShell(#[from] tauri_plugin_shell::Error),
    #[error("Update is already running")]
    UpdateAlreadyRunning,
    #[error("Shell command is already running, please restart the app")]
    ShellCommandAlreadyRunning,
    #[error("Shell command failed")]
    ShellCommandFailure(Vec<String>),
}

#[tauri::command]
pub fn git_status(app: AppHandle, name: String) -> GitStatus {
    GitStatus::new(name, &app)
}

#[derive(Clone, Serialize)]
pub enum GitUpdateStatus {
    Pending,
    PrepareStarted,
    CheckoutStarted,
    CloneStarted,
    Err(String),
    Ok,
}

pub struct GitUpdate {
    status: GitUpdateStatus,
    rx: Receiver<GitUpdateStatus>,
}

#[tauri::command]
pub fn git_update(app: AppHandle, name: String, repo: String, branch: String) -> Result<(), Error> {
    if app
        .state::<AppState>()
        .git_update
        .lock()
        .expect("Cannot acquire lock for git update rx")
        .is_some()
    {
        return Err(Error::UpdateAlreadyRunning);
    }

    let path = git_path(name.as_str(), app.app_handle());
    std::fs::remove_dir_all(path.as_path()).map_err(|err| Error::Io(err.to_string()))?;

    let (tx, rx) = channel::<GitUpdateStatus>();
    app.state::<AppState>()
        .git_update
        .lock()
        .expect("Cannot acquire lock for git update rx")
        .replace(GitUpdate {
            status: GitUpdateStatus::Pending,
            rx,
        });

    spawn_blocking(move || {
        tx.send(GitUpdateStatus::PrepareStarted)
            .expect("Cannot send message through git update channel");

        match gix::prepare_clone(repo, path)
            .map_err(|err| err.to_string())
            .and_then(|prepare| {
                prepare
                    .with_ref_name(Some(branch.as_str()))
                    .map_err(|_| "Invalid branch name".to_string())
            }) {
            Ok(mut prepare_fetch) => {
                tx.send(GitUpdateStatus::CheckoutStarted)
                    .expect("Cannot send message through git update channel");

                match prepare_fetch
                    .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
                {
                    Ok((mut prepare_checkout, _)) => {
                        tx.send(GitUpdateStatus::CloneStarted)
                            .expect("Cannot send message through git update channel");

                        match prepare_checkout
                            .main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
                        {
                            Ok(_) => {
                                tx.send(GitUpdateStatus::Ok)
                                    .expect("Cannot send message through git update channel");
                            }
                            Err(err) => {
                                tx.send(GitUpdateStatus::Err(err.to_string()))
                                    .expect("Cannot send message through git update channel");
                            }
                        }
                    }
                    Err(err) => {
                        tx.send(GitUpdateStatus::Err(err.to_string()))
                            .expect("Cannot send message through git update channel");
                    }
                }
            }
            Err(err) => {
                tx.send(GitUpdateStatus::Err(err))
                    .expect("Cannot send message through git update channel");
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub fn git_update_status(app: AppHandle) -> GitUpdateStatus {
    let state = app.state::<AppState>();
    let mut update = state
        .git_update
        .lock()
        .expect("Cannot acquire lock for git update rx");

    let status = if let Some(update) = update.as_mut() {
        match update.rx.try_recv() {
            Ok(status) => {
                update.status = status;
            }
            Err(TryRecvError::Disconnected) => update.status = GitUpdateStatus::Ok,
            _ => {}
        };
        update.status.clone()
    } else {
        GitUpdateStatus::Ok
    };

    if matches!(status, GitUpdateStatus::Ok | GitUpdateStatus::Err(_)) {
        update.take();
    }

    status
}

#[tauri::command]
pub async fn git_launch(
    app: AppHandle,
    name: String,
    bind_address: String,
) -> Result<String, Error> {
    if app
        .state::<AppState>()
        .git_command_child
        .lock()
        .expect("Cannot acquire lock of app state")
        .is_some()
    {
        return Err(Error::ShellCommandAlreadyRunning);
    }

    let static_server_command = app
        .shell()
        .sidecar("static-server")
        .expect("static server not found")
        .args([
            "--parent-pid",
            std::process::id().to_string().as_str(),
            "--bind-address",
            bind_address.as_str(),
            "--base-path",
            git_path(name.as_str(), app.app_handle())
                .to_str()
                .expect("Invalid git base path"),
        ]);
    let (mut rx, child) = static_server_command.spawn()?;

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
                                .git_command_child
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
