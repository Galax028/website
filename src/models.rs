use sqlx::Error as SqlxError;

pub(crate) mod projects;

pub(crate) type ModelResult<T> = Result<T, SqlxError>;
