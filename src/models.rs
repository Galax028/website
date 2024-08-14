use sqlx::{sqlite::SqliteRow, Error as SqlxError, FromRow, Row, SqlitePool};

mod projects;

pub(crate) type ModelResult<T> = Result<T, SqlxError>;

pub(crate) trait Read<R = SqliteRow>
where
    Self: Sized + for<'a> FromRow<'a, R> + Send + Sync + 'static,
    R: Sized + Row,
{
    async fn read(pool: &SqlitePool) -> ModelResult<Self> {
        unimplemented!()
    }
}
