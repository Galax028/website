#![warn(clippy::pedantic)]

mod config;
mod routes;
mod templating;

use axum::Router;
use config::AppConfig;
use http::{header, Method};
use minijinja::{path_loader, Environment};
use minijinja_autoreload::AutoReloader;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};
use std::{path::PathBuf, str::FromStr, sync::Arc, time::Duration};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, normalize_path::NormalizePathLayer};

/// Global state for the application.
#[derive(Clone)]
pub(crate) struct AppState {
    config: AppConfig,
    pool: SqlitePool,
    templater: Arc<AutoReloader>,
}

#[tokio::main]
async fn main() {
    let config = AppConfig::new();

    let pool = SqlitePoolOptions::new()
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(
            SqliteConnectOptions::from_str(&config.database_url)
                .expect("Failed to parse DATABASE_URL")
                .analysis_limit(1000)
                .journal_mode(SqliteJournalMode::Wal)
                .optimize_on_close(true, None),
        )
        .await
        .expect("Failed to crate a database connection pool");

    let app_state = AppState {
        config: config.clone(),
        pool,
        templater: Arc::new(AutoReloader::new(|notifier| {
            let templates_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/frontend/src");
            let mut templater = Environment::new();
            templater.set_loader(path_loader(&templates_dir));
            notifier.watch_path(templates_dir, true);

            Ok(templater)
        })),
    };

    let app = Router::new()
        .merge(routes::register(&config.static_root))
        .with_state(app_state)
        .layer(
            ServiceBuilder::new()
                .layer(NormalizePathLayer::trim_trailing_slash())
                .layer(
                    CorsLayer::new()
                        .allow_origin(config.cors_origins.clone())
                        .allow_methods([
                            Method::GET,
                            Method::POST,
                            Method::PUT,
                            Method::DELETE,
                            Method::PATCH,
                        ])
                        .allow_headers([
                            header::ACCEPT_ENCODING,
                            header::ACCEPT,
                            header::AUTHORIZATION,
                            header::CONTENT_TYPE,
                            header::COOKIE,
                            header::USER_AGENT,
                            // Track client IPs to count views
                            // HeaderName::from_static("CF-Connecting-IP"),
                        ])
                        .allow_credentials(true),
                ),
        );

    let listener = TcpListener::bind((config.host, config.port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
