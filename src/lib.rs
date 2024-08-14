pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod models;
pub(crate) mod routes;
pub(crate) mod state;
pub(crate) mod templating;

pub use config::AppConfig;
pub use routes::register_routes;
pub use state::AppState;
