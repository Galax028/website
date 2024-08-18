use serde::Serialize;
use sqlx::Error as SqlxError;

pub(crate) mod blog;
pub(crate) mod project;
pub(crate) mod tag;

pub(crate) type ModelResult<T> = Result<T, SqlxError>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Pagination {
    pub current_page: u64,
    pub last_page: u64,
    pub total_items: u64,
}
