use std::path::PathBuf;

use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn router(base_path: PathBuf) -> Router {
    Router::new().fallback_service(
        ServeDir::new(base_path.clone()).fallback(ServeFile::new(base_path.join("index.html"))),
    )
}
