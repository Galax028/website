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
            let filetype = file.file_type().await?;
            let filename = file.file_name().into_string().unwrap();

            println!("encountered {}", filename);
            println!("comp {}", filename.eq_ignore_ascii_case(".html"));

            // Iterate over to the next item if the current item is not a HTML file
            if !(filetype.is_file() && filename.eq_ignore_ascii_case(".html")) {
                continue;
            }

            let template = String::from_utf8(fs::read(file.path()).await?).unwrap();
            self.templater
                .add_template_owned(filename, template)
                .unwrap();
        }

        println!("loaded templates");
        Ok(self)
    }
}
