use dotenvy::dotenv;
use http::HeaderValue;
use std::{env::var, fs, net::IpAddr, path::PathBuf};

/// Global configuration options for the application.
#[derive(Clone, Debug)]
pub(crate) struct AppConfig {
    pub cors_origins: Vec<HeaderValue>,
    pub database_url: String,
    pub host: IpAddr,
    pub port: u16,
    pub static_root: PathBuf,
}

impl AppConfig {
    /// Creates a new `AppConfig` by loading environment variables.
    pub fn new() -> AppConfig {
        dotenv().expect("Failed to load environment variables");

        let cors_origins = var("CORS_ORIGINS")
            .unwrap_or_default()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");
        let host = var("HOST")
            .expect("HOST must be set")
            .parse::<IpAddr>()
            .expect("HOST must be a valid IP address");
        let port = var("PORT")
            .expect("PORT must be set")
            .parse::<u16>()
            .expect("PORT must be a number between 1 and 65535");
        let static_root = var("STATIC_ROOT")
            .expect("STATIC_ROOT must be set")
            .parse::<PathBuf>()
            .unwrap(); // Unwrap-safe: Error type from result is `Infallible`.

        if !fs::metadata(&static_root)
            .expect("STATIC_ROOT does not exist or cannot be accessed")
            .is_dir()
        {
            panic!("STATIC_ROOT must be a directory");
        }

        AppConfig {
            cors_origins,
            database_url,
            host,
            port,
            static_root,
        }
    }
}
