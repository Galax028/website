use crate::{
    error::HandlerResult,
    models::{blog::Blog, project::Project},
    templating::{
        self, ErrorTemplateContext, IndexTemplateContext, ProjectsTemplateContext, Template,
        TemplateMeta,
    },
    AppState,
};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use http::StatusCode;
use std::{collections::HashSet, path::Path};
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
        recent_posts: Blog::get_recent_blogs(&state.pool).await?,
    };
    let result = templating::render_template(&state.templater, Template::Index, context)?;

    Ok(Html(result))
}

async fn render_projects(State(state): State<AppState>) -> HandlerResult<Html<String>> {
    let context = ProjectsTemplateContext {
        meta: TemplateMeta::generate_with_non_direct_deps(
            "Projects | ",
            &state.config.static_root,
            HashSet::from(["styles/projects.css"]),
            HashSet::default(),
        )
        .await?,
        projects: Project::get_all_projects(&state.pool).await?,
    };
    let result = templating::render_template(&state.templater, Template::Projects, context)?;

    Ok(Html(result))
}

pub(super) async fn render_not_found(
    State(state): State<AppState>,
) -> HandlerResult<impl IntoResponse> {
    let context = ErrorTemplateContext {
        meta: TemplateMeta::generate("404 Not Found | ", &state.config.static_root).await?,
        error_code: 404,
        error_description: "Not Found ¯\\_(ツ)_/¯".to_string(),
    };
    let result = templating::render_template(&state.templater, Template::Error, context)?;

    Ok((StatusCode::NOT_FOUND, Html(result)).into_response())
}
