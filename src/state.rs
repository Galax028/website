use crate::AppConfig;
use minijinja::Environment as JinjaEnvironment;
use sqlx::SqlitePool;

#[cfg(debug_assertions)]
use minijinja_autoreload::AutoReloader as JinjaAutoReloader;
#[cfg(debug_assertions)]
use std::{path::PathBuf, sync::Arc};

#[cfg(not(debug_assertions))]
use tokio::fs;

#[cfg(debug_assertions)]
/// Global state for the application.
#[derive(Clone)]
pub(crate) struct AppState {
    pub config: AppConfig,
    pub pool: SqlitePool,
    pub templater: Arc<JinjaAutoReloader>,
}

#[cfg(debug_assertions)]
impl AppState {
    /// Creates a new `AppState`.
    pub(crate) fn new(config: AppConfig, pool: SqlitePool) -> Self {
        AppState {
            config,
            pool,
            templater: Arc::new(JinjaAutoReloader::new(|notifier| {
                let templates_dir =
                    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/frontend/src");
                let mut templater = JinjaEnvironment::new();
                templater.set_loader(minijinja::path_loader(&templates_dir));
                notifier.watch_path(templates_dir, true);

                Ok(templater)
            })),
        }
    }
}

#[cfg(not(debug_assertions))]
/// Global state for the application.
#[derive(Clone)]
pub(crate) struct AppState {
    pub config: AppConfig,
    pub pool: SqlitePool,
    pub templater: JinjaEnvironment<'static>,
}

#[cfg(not(debug_assertions))]
impl AppState {
    /// Creates a new `AppState`.
    pub(crate) fn new(config: AppConfig, pool: SqlitePool) -> Self {
        AppState {
            config,
            pool,
            templater: JinjaEnvironment::new(),
        }
    }

    /// Load Jinja templates into the templater.
    pub(crate) async fn load_templates(mut self) -> Result<Self, std::io::Error> {
        let mut templates_dir = fs::read_dir(&self.config.static_root).await?;

        while let Some(file) = templates_dir.next_entry().await? {
            let file_path = file.path();
            let file_type = file.file_type().await?;
            let file_name = file.file_name().into_string().unwrap();

            // Iterate over to the next item if the current item is not a HTML file
            if !(file_type.is_file()
                && file_path
                    .extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("html")))
            {
                continue;
            }

            let template = String::from_utf8(fs::read(&file_path).await?).unwrap();
            self.templater
                .add_template_owned(file_name, template)
                .unwrap();
        }

        Ok(self)
    }
}
