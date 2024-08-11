use crate::AppState;
use axum::{routing::get, Router};
use std::path::Path;
use tower_http::services::{ServeDir, ServeFile};

mod frontend;

pub(crate) fn register(static_root: &Path) -> Router<AppState> {
    Router::new()
        .route("/", get(frontend::render_index))
        .route_service(
            "/favicon-dark-mode.png",
            ServeFile::new(static_root.join("favicon-dark-mode.png")),
        )
        .route_service(
            "/favicon-light-mode.png",
            ServeFile::new(static_root.join("favicon-light-mode.png")),
        )
        .nest_service("/assets", ServeDir::new(static_root.join("assets")))
        .fallback(get(frontend::render_not_found))
}

// #[cfg(not(debug_assertions))]
// pub(crate) fn register(static_root: &Path) -> Router<AppState> {
//     Router::new()
//         .route("/hello-world", get(frontend::hello_world))
//         .route_service("/", ServeFile::new(static_root.join("index.html")))
//         .route_service(
//             "/favicon-dark-mode.png",
//             ServeFile::new(static_root.join("favicon-dark-mode.png")),
//         )
//         .route_service(
//             "/favicon-light-mode.png",
//             ServeFile::new(static_root.join("favicon-light-mode.png")),
//         )
//         .nest_service("/assets", ServeDir::new(static_root.join("assets")))
//         .fallback_service(ServeFile::new(static_root.join("not-found.html")))
// }
