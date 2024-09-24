use std::sync::mpsc::{channel, Receiver, TryRecvError};

use regex::Regex;
use serde::Serialize;
use tauri::{async_runtime::spawn_blocking, AppHandle, Manager};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};

use crate::{models::noname::NonameStatus, utils::fs::noname_path, AppState};

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
pub fn noname_status(app: AppHandle) -> NonameStatus {
    NonameStatus::new(&app)
}

#[derive(Clone, Serialize)]
pub enum NonameUpdateStatus {
    Pending,
    PrepareStarted,
    CheckoutStarted,
    CloneStarted,
    Err(String),
    Ok,
}

pub struct NonameUpdate {
    status: NonameUpdateStatus,
    rx: Receiver<NonameUpdateStatus>,
}

#[tauri::command]
pub fn noname_update(app: AppHandle, repo: String, branch: String) -> Result<(), Error> {
    if app
        .state::<AppState>()
        .noname_update
        .lock()
        .expect("Cannot acquire lock for noname update rx")
        .is_some()
    {
        return Err(Error::UpdateAlreadyRunning);
    }

    let path = noname_path(app.app_handle());
    std::fs::remove_dir_all(path.as_path()).map_err(|err| Error::Io(err.to_string()))?;

    let (tx, rx) = channel::<NonameUpdateStatus>();
    app.state::<AppState>()
        .noname_update
        .lock()
        .expect("Cannot acquire lock for noname update rx")
        .replace(NonameUpdate {
            status: NonameUpdateStatus::Pending,
            rx,
        });

    spawn_blocking(move || {
        tx.send(NonameUpdateStatus::PrepareStarted)
            .expect("Cannot send message through noname update channel");

        match gix::prepare_clone(repo, path)
            .map_err(|err| err.to_string())
            .and_then(|prepare| {
                prepare
                    .with_ref_name(Some(branch.as_str()))
                    .map_err(|_| "Invalid branch name".to_string())
            }) {
            Ok(mut prepare_fetch) => {
                tx.send(NonameUpdateStatus::CheckoutStarted)
                    .expect("Cannot send message through noname update channel");

                match prepare_fetch
                    .fetch_then_checkout(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
                {
                    Ok((mut prepare_checkout, _)) => {
                        tx.send(NonameUpdateStatus::CloneStarted)
                            .expect("Cannot send message through noname update channel");

                        match prepare_checkout
                            .main_worktree(gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
                        {
                            Ok(_) => {
                                tx.send(NonameUpdateStatus::Ok)
                                    .expect("Cannot send message through noname update channel");
                            }
                            Err(err) => {
                                tx.send(NonameUpdateStatus::Err(err.to_string()))
                                    .expect("Cannot send message through noname update channel");
                            }
                        }
                    }
                    Err(err) => {
                        tx.send(NonameUpdateStatus::Err(err.to_string()))
                            .expect("Cannot send message through noname update channel");
                    }
                }
            }
            Err(err) => {
                tx.send(NonameUpdateStatus::Err(err))
                    .expect("Cannot send message through noname update channel");
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub fn noname_update_status(app: AppHandle) -> NonameUpdateStatus {
    let state = app.state::<AppState>();
    let mut update = state
        .noname_update
        .lock()
        .expect("Cannot acquire lock for noname update rx");

    let status = if let Some(update) = update.as_mut() {
        match update.rx.try_recv() {
            Ok(status) => {
                update.status = status;
            }
            Err(TryRecvError::Disconnected) => update.status = NonameUpdateStatus::Ok,
            _ => {}
        };
        update.status.clone()
    } else {
        NonameUpdateStatus::Ok
    };

    if matches!(status, NonameUpdateStatus::Ok | NonameUpdateStatus::Err(_)) {
        update.take();
    }

    status
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
