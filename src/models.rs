use sqlx::Error as SqlxError;

pub(crate) mod blog;
pub(crate) mod project;

pub(crate) type ModelResult<T> = Result<T, SqlxError>;
