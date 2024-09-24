use std::{env, net::SocketAddr, path::Path};

use tokio::net::TcpListener;

mod controllers;

#[tokio::main]
async fn main() {
    let base_path =
        Path::new(env::args().next().expect("No base path provided").as_str()).to_path_buf();

    let addr = SocketAddr::from(([127, 0, 0, 1], 44444));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Cannot bind http server");
    let app = controllers::router(base_path);
    axum::serve(listener, app).await.unwrap();
}
