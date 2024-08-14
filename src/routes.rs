use crate::AppState;
use axum::{routing::get, Router};
use std::path::Path;

mod frontend;

pub fn register_routes<P: AsRef<Path>>(static_root: P) -> Router<AppState> {
    Router::new()
        .merge(frontend::register(static_root))
        .fallback(get(frontend::render_not_found))
}
