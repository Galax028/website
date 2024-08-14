use anyhow::Result;
use serde::Serialize;
use std::path::Path;

#[cfg(debug_assertions)]
use minijinja_autoreload::AutoReloader as JinjaAutoReloader;

#[cfg(not(debug_assertions))]
use serde::Deserialize;
#[cfg(not(debug_assertions))]
use std::collections::HashMap;
#[cfg(not(debug_assertions))]
use std::path::PathBuf;
#[cfg(not(debug_assertions))]
use tokio::fs;

#[cfg(not(debug_assertions))]
use minijinja::Environment as JinjaEnvironment;

#[cfg(debug_assertions)]
static RENDER_MODE: &str = "development";

#[cfg(not(debug_assertions))]
static RENDER_MODE: &str = "production";

pub(crate) enum Template {
    Index,
    Error,
}

impl From<Template> for &str {
    fn from(template: Template) -> Self {
        match template {
            Template::Index => "index.html",
            Template::Error => "error.html",
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TemplateMeta {
    pub mode: &'static str,
    pub title: String,
    pub css_links: Vec<String>,
    pub script_tags: Vec<String>,
}

impl TemplateMeta {
    pub(crate) async fn generate<P: AsRef<Path>>(title: &str, static_root: P) -> Result<Self> {
        let static_root = static_root.as_ref();

        Ok(TemplateMeta {
            mode: RENDER_MODE,
            title: title.to_string(),
            css_links: get_css_links(static_root).await?,
            script_tags: get_script_tags(static_root).await?,
        })
    }
}

#[derive(Serialize)]
pub(crate) struct IndexTemplateContext {
    pub meta: TemplateMeta,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorTemplateContext {
    pub meta: TemplateMeta,
    pub error_code: u16,
    pub error_description: String,
}

#[cfg(not(debug_assertions))]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ViteManifestItem {
    file: PathBuf,
    #[serde(default)]
    is_entry: bool,
}

#[cfg(debug_assertions)]
async fn get_css_links<P: AsRef<Path>>(_: P) -> Result<Vec<String>> {
    Ok(Vec::new())
}

#[cfg(debug_assertions)]
async fn get_script_tags<P: AsRef<Path>>(_: P) -> Result<Vec<String>> {
    Ok(["@vite/client", "main.ts", "styles/global.css"]
        .iter()
        .map(|s| format!(r#"<script type="module" src="http://localhost:5173/{s}"></script>"#))
        .collect())
}

#[cfg(not(debug_assertions))]
async fn get_css_links<P: AsRef<Path>>(static_root: P) -> Result<Vec<String>> {
    let static_root = static_root.as_ref();
    let manifest_raw = fs::read_to_string(static_root.join(".vite/manifest.json")).await?;
    let vite_manifest = serde_json::from_str::<HashMap<String, ViteManifestItem>>(&manifest_raw)?;

    Ok(vite_manifest
        .into_values()
        .filter(|item| {
            item.is_entry
                && item
                    .file
                    .extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("css"))
        })
        .map(|item| {
            format!(
                r#"<link rel="stylesheet" href="/{}" />"#,
                item.file.display()
            )
        })
        .collect())
}

#[cfg(not(debug_assertions))]
async fn get_script_tags<P: AsRef<Path>>(static_root: P) -> Result<Vec<String>> {
    let static_root = static_root.as_ref();
    let manifest_raw = fs::read_to_string(static_root.join(".vite/manifest.json")).await?;
    let vite_manifest = serde_json::from_str::<HashMap<String, ViteManifestItem>>(&manifest_raw)?;

    Ok(vite_manifest
        .into_values()
        .filter(|item| {
            item.is_entry
                && item
                    .file
                    .extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("js"))
        })
        .map(|item| {
            format!(
                r#"<script type="module" src="/{}"></script>"#,
                item.file.display()
            )
        })
        .collect())
}

#[cfg(debug_assertions)]
pub(crate) fn render_template<C: Serialize>(
    templater: &JinjaAutoReloader,
    name: Template,
    context: C,
) -> Result<String> {
    let templater = templater.acquire_env()?;
    let template = templater.get_template(name.into())?;

    Ok(template.render(context)?)
}

#[cfg(not(debug_assertions))]
pub(crate) fn render_template<C: Serialize>(
    templater: &JinjaEnvironment<'static>,
    name: &str,
    context: C,
) -> Result<String> {
    let template = templater.get_template(name.into())?;

    Ok(template.render(context)?)
}
