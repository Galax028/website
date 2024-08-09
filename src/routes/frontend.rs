use crate::AppState;
use axum::{extract::State, response::IntoResponse};
use http::StatusCode;

pub(super) async fn hello_world(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("{state:?}");

    Ok("Hello, world!")
}
