use crate::AppState;
use axum::{routing::get, Router};
use std::path::Path;
use tower_http::services::ServeFile;

mod frontend;

pub(crate) fn register(static_root: &Path) -> Router<AppState> {
    Router::new()
        .route("/hello-world", get(frontend::hello_world))
        .route_service("/", ServeFile::new(static_root.join("index.html")))
}
