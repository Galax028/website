use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TemplateMeta {
    pub mode: &'static str,
    pub title: String,
    pub css_files: Vec<String>,
    pub js_files: Vec<String>,
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ViteManifestItem {
    file: String,
    #[serde(default)]
    is_entry: bool,
}

// TODO: Proper error handling
pub(crate) async fn get_file_chunks(
    static_root: &Path,
    extension: &str,
) -> Result<Vec<String>, std::io::Error> {
    let manifest_raw = fs::read_to_string(static_root.join(".vite/manifest.json")).await?;
    let vite_manifest =
        serde_json::from_str::<HashMap<String, ViteManifestItem>>(&manifest_raw).unwrap();

    Ok(vite_manifest
        .into_values()
        .filter(|item| item.is_entry && item.file.ends_with(extension))
        .map(|item| item.file)
        .collect())
}
