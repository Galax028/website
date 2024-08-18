#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

pub(crate) mod config;
pub(crate) mod error;
pub(crate) mod models;
pub(crate) mod routes;
pub(crate) mod state;
pub(crate) mod templating;

pub use config::Config;
pub use routes::root;
pub use state::AppState;
