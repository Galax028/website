use crate::models::{blog::RecentBlog, project::Project};
use anyhow::Result;
use serde::Serialize;
use std::collections::HashSet;
use std::path::Path;

#[cfg(debug_assertions)]
use minijinja_autoreload::AutoReloader as JinjaAutoReloader;
#[cfg(debug_assertions)]
use tracing::info;

#[cfg(not(debug_assertions))]
use serde::Deserialize;
#[cfg(not(debug_assertions))]
use std::collections::HashMap;
#[cfg(not(debug_assertions))]
use std::path::PathBuf;
#[cfg(not(debug_assertions))]
use std::sync::LazyLock;
#[cfg(not(debug_assertions))]
use tokio::fs;

#[cfg(not(debug_assertions))]
use minijinja::Environment as JinjaEnvironment;

#[cfg(debug_assertions)]
static RENDER_MODE: &str = "development";

#[cfg(not(debug_assertions))]
static RENDER_MODE: &str = "production";

#[cfg(not(debug_assertions))]
static DIRECT_DEPS: LazyLock<HashSet<&str>> =
    LazyLock::new(|| HashSet::from(["main.ts", "styles/global.css"]));

#[derive(Debug)]
pub(crate) enum Template {
    Index,
    Projects,
    Error,
}

impl From<Template> for &str {
    fn from(template: Template) -> Self {
        match template {
            Template::Index => "index.html",
            Template::Projects => "projects.html",
            Template::Error => "error.html",
        }
    }
}

#[derive(Debug, Serialize)]
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

    #[allow(unused_variables)]
    #[cfg(debug_assertions)]
    pub(crate) async fn generate_with_non_direct_deps<P: AsRef<Path>>(
        title: &'static str,
        static_root: P,
        css_deps: HashSet<&'static str>,
        js_deps: HashSet<&'static str>,
    ) -> Result<Self> {
        Self::generate(title, static_root).await
    }

    #[cfg(not(debug_assertions))]
    pub(crate) async fn generate_with_non_direct_deps<P: AsRef<Path>>(
        title: &'static str,
        static_root: P,
        css_deps: HashSet<&'static str>,
        js_deps: HashSet<&'static str>,
    ) -> Result<Self> {
        let static_root = static_root.as_ref();

        Ok(TemplateMeta {
            mode: RENDER_MODE,
            title: title.to_string(),
            css_links: [
                get_css_links(static_root).await?,
                get_non_direct_css_links(static_root, css_deps).await?,
            ]
            .concat(),
            script_tags: [
                get_script_tags(static_root).await?,
                get_non_direct_script_tags(static_root, js_deps).await?,
            ]
            .concat(),
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IndexTemplateContext {
    pub meta: TemplateMeta,
    pub recent_posts: Vec<RecentBlog>,
}

#[derive(Debug, Serialize)]
pub(crate) struct ProjectsTemplateContext {
    pub meta: TemplateMeta,
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize)]
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
    src: String,
    #[serde(default)]
    is_entry: bool,
}

#[allow(clippy::unused_async, unused_variables)]
#[cfg(debug_assertions)]
async fn get_css_links<P: AsRef<Path>>(static_root: P) -> Result<Vec<String>> {
    Ok(Vec::new())
}

#[allow(clippy::unused_async, unused_variables)]
#[cfg(debug_assertions)]
async fn get_script_tags<P: AsRef<Path>>(static_root: P) -> Result<Vec<String>> {
    Ok(["@vite/client", "main.ts", "styles/global.css"]
        .iter()
        .map(|s| format!(r#"<script type="module" src="http://localhost:5173/{s}"></script>"#))
        .collect())
}

#[cfg(not(debug_assertions))]
async fn vite_manifest_filter<P, F>(
    static_root: P,
    file_extension: &'static str,
    predicate: F,
) -> Result<impl Iterator<Item = ViteManifestItem>>
where
    P: AsRef<Path>,
    F: Fn(&ViteManifestItem) -> bool,
{
    let static_root = static_root.as_ref();
    let manifest_raw = fs::read_to_string(static_root.join(".vite/manifest.json")).await?;
    let vite_manifest = serde_json::from_str::<HashMap<String, ViteManifestItem>>(&manifest_raw)?;

    Ok(vite_manifest.into_values().filter(move |item| {
        item.is_entry
            && item
                .file
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case(file_extension))
            && predicate(item)
    }))
}

#[cfg(not(debug_assertions))]
async fn get_css_links<P: AsRef<Path>>(static_root: P) -> Result<Vec<String>> {
    Ok(
        vite_manifest_filter(static_root, "css", |item| DIRECT_DEPS.contains(&*item.src))
            .await?
            .map(|item| {
                format!(
                    r#"<link rel="stylesheet" href="/{}" />"#,
                    item.file.display()
                )
            })
            .collect(),
    )
}

#[cfg(not(debug_assertions))]
async fn get_script_tags<P: AsRef<Path>>(static_root: P) -> Result<Vec<String>> {
    Ok(
        vite_manifest_filter(static_root, "js", |item| DIRECT_DEPS.contains(&*item.src))
            .await?
            .map(|item| {
                format!(
                    r#"<script type="module" src="/{}"></script>"#,
                    item.file.display()
                )
            })
            .collect(),
    )
}

#[cfg(not(debug_assertions))]
async fn get_non_direct_css_links<P: AsRef<Path>>(
    static_root: P,
    dependencies: HashSet<&'static str>,
) -> Result<Vec<String>> {
    Ok(
        vite_manifest_filter(static_root, "css", |item| dependencies.contains(&*item.src))
            .await?
            .map(|item| {
                format!(
                    r#"<link rel="stylesheet" href="/{}" />"#,
                    item.file.display()
                )
            })
            .collect(),
    )
}

#[cfg(not(debug_assertions))]
async fn get_non_direct_script_tags<P: AsRef<Path>>(
    static_root: P,
    dependencies: HashSet<&'static str>,
) -> Result<Vec<String>> {
    Ok(
        vite_manifest_filter(static_root, "js", |item| dependencies.contains(&*item.src))
            .await?
            .map(|item| {
                format!(
                    r#"<link rel="stylesheet" href="/{}" />"#,
                    item.file.display()
                )
            })
            .collect(),
    )
}

#[cfg(debug_assertions)]
#[tracing::instrument(skip(templater))]
pub(crate) fn render_template<C: Serialize + std::fmt::Debug>(
    templater: &JinjaAutoReloader,
    name: Template,
    context: C,
) -> Result<String> {
    let templater = templater.acquire_env()?;
    let template = templater.get_template(name.into())?;

    let res = template.render(context)?;
    info!("render template");

    Ok(res)
}

#[cfg(not(debug_assertions))]
pub(crate) fn render_template<C: Serialize>(
    templater: &JinjaEnvironment<'static>,
    name: Template,
    context: C,
) -> Result<String> {
    let template = templater.get_template(name.into())?;

    Ok(template.render(context)?)
}
