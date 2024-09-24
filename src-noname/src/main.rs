use std::{net::SocketAddr, path::Path, str::FromStr};

use clap::{command, Parser};
use tokio::net::TcpListener;

mod controllers;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(long)]
    bind_ip: String,

    #[arg(long)]
    base_path: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let addr = SocketAddr::from_str(args.bind_ip.as_str()).expect("Invalid bind ip");
    let listener = TcpListener::bind(addr)
        .await
        .expect("Cannot bind http server");
    println!(
        "Listening on {}",
        listener.local_addr().expect("Cannot get local address")
    );
    let app = controllers::router(Path::new(args.base_path.as_str()).to_path_buf());
    axum::serve(listener, app).await.unwrap();
}
