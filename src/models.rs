use sqlx::Error as SqlxError;

mod projects;

pub(crate) type ModelResult<T> = Result<T, SqlxError>;
