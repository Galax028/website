use crate::{
    templating::{get_file_chunks, ErrorTemplateContext, IndexTemplateContext, TemplateMeta},
    AppState,
};
use axum::{extract::State, response::Html};
use http::StatusCode;

#[cfg(debug_assertions)]
static RENDER_MODE: &str = "development";

#[cfg(not(debug_assertions))]
static RENDER_MODE: &str = "production";

pub(super) async fn render_index(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let context = IndexTemplateContext {
        meta: TemplateMeta {
            mode: RENDER_MODE,
            title: String::default(),
            css_files: get_file_chunks(&state.config.static_root, ".css")
                .await
                .unwrap(),
            js_files: get_file_chunks(&state.config.static_root, ".js")
                .await
                .unwrap(),
        },
    };
    let templater = state.templater.acquire_env().unwrap();
    let template = templater.get_template("index.html").unwrap().clone();

    let result = template.render(context).unwrap();

    Ok(Html(result))
}

pub(super) async fn render_not_found(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let context = ErrorTemplateContext {
        meta: TemplateMeta {
            mode: RENDER_MODE,
            title: "404 Not Found |".to_string(),
            css_files: get_file_chunks(&state.config.static_root, ".css")
                .await
                .unwrap(),
            js_files: get_file_chunks(&state.config.static_root, ".js")
                .await
                .unwrap(),
        },
        error_code: 404,
        error_description: "Not Found ¯\\_(ツ)_/¯".to_string(),
    };
    let templater = state.templater.acquire_env().unwrap();
    let template = templater.get_template("error.html").unwrap();

    let result = template.render(context).unwrap();

    Ok(Html(result))
}
