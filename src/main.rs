#![warn(clippy::pedantic)]

mod config;
mod routes;
mod state;
mod templating;

use axum::Router;
use http::{header, Method};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use std::{str::FromStr, time::Duration};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, normalize_path::NormalizePathLayer};

pub(crate) use config::AppConfig;
pub(crate) use state::AppState;

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

    // #[cfg(debug_assertions)]
    // let app_state = AppState::new(config.clone(), pool);

    // #[cfg(not(debug_assertions))]
    let app_state = AppState::new(config.clone(), pool)
        .load_templates()
        .await
        .expect("Failed to load templates");

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
