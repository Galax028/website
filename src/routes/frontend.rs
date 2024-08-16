use std::path::Path;

use crate::{
    error::HandlerResult,
    templating::{self, ErrorTemplateContext, IndexTemplateContext, Template, TemplateMeta},
    AppState,
};
use axum::{extract::State, response::Html, routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

pub(super) fn register<P: AsRef<Path>>(static_root: P) -> Router<AppState> {
    let static_root = static_root.as_ref();

    Router::new()
        .route("/", get(render_index))
        .route("/projects", get(render_projects))
        .route_service(
            "/favicon-dark-mode.png",
            ServeFile::new(static_root.join("favicon-dark-mode.png")),
        )
        .route_service(
            "/favicon-light-mode.png",
            ServeFile::new(static_root.join("favicon-light-mode.png")),
        )
        .route_service(
            "/robots.txt",
            ServeFile::new(static_root.join("robots.txt")),
        )
        .nest_service("/assets", ServeDir::new(static_root.join("assets")))
}

async fn render_index(State(state): State<AppState>) -> HandlerResult<Html<String>> {
    let context = IndexTemplateContext {
        meta: TemplateMeta::generate("", &state.config.static_root).await?,
    };
    let result = templating::render_template(&state.templater, Template::Index, context)?;

    Ok(Html(result))
}

async fn render_projects(State(state): State<AppState>) -> HandlerResult<Html<String>> {
    let context = IndexTemplateContext {
        meta: TemplateMeta::generate("Projects | ", &state.config.static_root).await?,
    };
    let result = templating::render_template(&state.templater, Template::Projects, context)?;

    Ok(Html(result))
}

pub(super) async fn render_not_found(State(state): State<AppState>) -> HandlerResult<Html<String>> {
    let context = ErrorTemplateContext {
        meta: TemplateMeta::generate("404 Not Found | ", &state.config.static_root).await?,
        error_code: 404,
        error_description: "Not Found ¯\\_(ツ)_/¯".to_string(),
    };
    let result = templating::render_template(&state.templater, Template::Error, context)?;

    Ok(Html(result))
}
