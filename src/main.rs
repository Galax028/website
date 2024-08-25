#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

use anyhow::{Context, Result};
use axum::Router;
use http::{header, Method};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use std::{str::FromStr, time::Duration};
use tokio::{net::TcpListener, signal};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    normalize_path::NormalizePathLayer,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};
use website::{root, AppState, Config};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            #[cfg(debug_assertions)]
            EnvFilter::builder()
                .parse("website=debug,sqlx=debug,axum::rejection=trace,tower_http=debug")?,
            #[cfg(not(debug_assertions))]
            EnvFilter::builder()
                .parse("website=info,sqlx=error,axum::rejection=error,tower_http=error")?,
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .context("Failed to initialise tracing subscriber")?;

    let config = Config::new().context("Failed to parse config")?;

    let pool = SqlitePoolOptions::new()
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(
            SqliteConnectOptions::from_str(&config.database_url)
                .context("Failed to parse DATABASE_URL")?
                .analysis_limit(1000)
                .journal_mode(SqliteJournalMode::Wal)
                .optimize_on_close(true, None),
        )
        .await
        .context("Failed to crate a database connection pool")?;

    #[cfg(debug_assertions)]
    let app_state = AppState::new(config.clone(), pool);

    #[cfg(not(debug_assertions))]
    let app_state = AppState::new(config.clone(), pool)
        .load_templates()
        .await
        .context("Failed to load templates")?;

    let app = Router::new()
        .merge(root(&config.static_root))
        .with_state(app_state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new()))
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
                        ])
                        .allow_credentials(true),
                ),
        );

    let listener = TcpListener::bind((config.host, config.port)).await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("Failed to serve the application")?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .context("Failed to install SIGINT handler")
            .unwrap();
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .context("Failed to install SIGTERM handler")
            .unwrap()
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {
            info!("received SIGINT, stopping server");
        },
        () = terminate => {},
    }
}
