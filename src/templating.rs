use minijinja::Error as JinjaError;
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TemplateMeta {
    pub mode: &'static str,
    pub title: String,
    pub css_links: Vec<String>,
    pub script_tags: Vec<String>,
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
// TODO: Proper error handling
pub(crate) async fn get_css_links(_: &Path) -> Result<Vec<String>, std::io::Error> {
    Ok(Vec::new())
}

#[cfg(debug_assertions)]
// TODO: Proper error handling
pub(crate) async fn get_script_tags(_: &Path) -> Result<Vec<String>, std::io::Error> {
    Ok(["@vite/client", "main.ts", "styles/global.css"]
        .iter()
        .map(|s| format!(r#"<script type="module" src="http://localhost:5173/{s}"></script>"#))
        .collect())
}

#[cfg(not(debug_assertions))]
// TODO: Proper error handling
pub(crate) async fn get_css_links(static_root: &Path) -> Result<Vec<String>, std::io::Error> {
    let manifest_raw = fs::read_to_string(static_root.join(".vite/manifest.json")).await?;
    let vite_manifest =
        serde_json::from_str::<HashMap<String, ViteManifestItem>>(&manifest_raw).unwrap();

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
// TODO: Proper error handling
pub(crate) async fn get_script_tags(static_root: &Path) -> Result<Vec<String>, std::io::Error> {
    let manifest_raw = fs::read_to_string(static_root.join(".vite/manifest.json")).await?;
    let vite_manifest =
        serde_json::from_str::<HashMap<String, ViteManifestItem>>(&manifest_raw).unwrap();

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
pub(crate) fn render_template<C>(
    templater: &JinjaAutoReloader,
    name: &str,
    context: C,
) -> Result<String, JinjaError>
where
    C: Serialize,
{
    let templater = templater.acquire_env().unwrap();
    let template = templater.get_template(&format!("{name}.html"))?;

    template.render(context)
}

#[cfg(not(debug_assertions))]
pub(crate) fn render_template<C>(
    templater: &JinjaEnvironment<'static>,
    name: &str,
    context: C,
) -> Result<String, JinjaError>
where
    C: Serialize,
{
    let template = templater.get_template(&format!("{name}.html"))?;

    template.render(context)
}
