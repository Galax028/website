use crate::{
    templating::{
        self, get_css_links, get_script_tags, ErrorTemplateContext, IndexTemplateContext,
        TemplateMeta,
    },
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
            css_links: get_css_links(&state.config.static_root).await.unwrap(),
            script_tags: get_script_tags(&state.config.static_root).await.unwrap(),
        },
    };
    let result = templating::render_template(&state.templater, "index", context).unwrap();

    Ok(Html(result))
}

pub(super) async fn render_not_found(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let context = ErrorTemplateContext {
        meta: TemplateMeta {
            mode: RENDER_MODE,
            title: "404 Not Found |".to_string(),
            css_links: get_css_links(&state.config.static_root).await.unwrap(),
            script_tags: get_script_tags(&state.config.static_root).await.unwrap(),
        },
        error_code: 404,
        error_description: "Not Found ¯\\_(ツ)_/¯".to_string(),
    };
    let result = templating::render_template(&state.templater, "error", context).unwrap();

    Ok(Html(result))
}
