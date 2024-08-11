use crate::AppState;
use axum::{extract::State, response::Html};
use http::StatusCode;
use minijinja::context;

#[cfg(debug_assertions)]
static RENDER_MODE: &str = "development";

#[cfg(not(debug_assertions))]
static RENDER_MODE: &str = "production";

pub(super) async fn render_index(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let templater = state.templater.acquire_env().unwrap();
    let template = templater.get_template("index.html").unwrap();

    let render = template
        .render(context! {
            mode => RENDER_MODE,
            title => "",
        })
        .unwrap();

    Ok(Html(render))
}

pub(super) async fn render_not_found(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let templater = state.templater.acquire_env().unwrap();
    let template = templater.get_template("error.html").unwrap();

    let render = template
        .render(context! {
            mode => RENDER_MODE,
            title => "404 Not Found | ",
        })
        .unwrap();

    Ok(Html(render))
}
