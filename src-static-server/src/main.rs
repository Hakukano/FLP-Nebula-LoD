use std::{net::SocketAddr, path::Path, str::FromStr};

use clap::{command, Parser};
use tokio::net::TcpListener;

mod controllers;
mod watcher;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(long)]
    parent_pid: u32,

    #[arg(long)]
    bind_address: String,

    #[arg(long)]
    base_path: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let addr = SocketAddr::from_str(args.bind_address.as_str()).expect("Invalid bind address");
    let listener = TcpListener::bind(addr)
        .await
        .expect("Cannot bind http server");
    println!(
        "Listening on {}",
        listener.local_addr().expect("Cannot get local address")
    );
    let app = controllers::router(Path::new(args.base_path.as_str()).to_path_buf());
    tokio::spawn(async move { axum::serve(listener, app).await });

    watcher::run(args.parent_pid).await;
}
