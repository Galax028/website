use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::Serialize;
use thiserror::Error;

#[cfg(debug_assertions)]
use std::error::Error as _;

pub(crate) type HandlerResult<T> = Result<T, AppError>;

#[derive(Error, Debug)]
pub(crate) enum AppError {
    #[allow(dead_code)]
    #[error("Not Found")]
    NotFound,

    #[error("Internal Server Error")]
    Sqlx(#[from] sqlx::Error),

    #[error("Internal Server Error")]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Serialize)]
pub(crate) struct AppErrorResponse {
    pub code: u16,
    pub description: String,
    pub source: String,
}

impl AppError {
    fn status_code(&self) -> u16 {
        match self {
            AppError::NotFound => 404,
            AppError::Sqlx(_) | AppError::Anyhow(_) => 500,
        }
    }
}

// TODO: Wrong configuration
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.status_code()).unwrap(),
            Json(AppErrorResponse {
                code: self.status_code(),
                description: self.to_string(),
                #[cfg(debug_assertions)]
                source: self.source().map_or(String::default(), ToString::to_string),
                #[cfg(not(debug_assertions))]
                source: "An unexpected error occurred while trying to process the request"
                    .to_string(),
            }),
        )
            .into_response()
    }
}
