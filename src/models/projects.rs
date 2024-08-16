use chrono::{DateTime, Utc};
use sqlx::{prelude::FromRow, query, SqlitePool};
use uuid::Uuid;

use super::ModelResult;

#[derive(FromRow)]
pub(crate) struct Project {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub starred: bool,
    pub showcase: Option<String>,
    pub repository: String,
}

impl Project {
    async fn create(
        pool: &SqlitePool,
        name: &str,
        description: &str,
        starred: bool,
        showcase: Option<&str>,
        repository: &str,
    ) -> ModelResult<()> {
        let id = Uuid::new_v4();
        query!(
            "
            INSERT INTO project (id, name, description, starred, showcase, repository)
            VALUES ($1, $2, $3, $4, $5, $6)
            ",
            id,
            name,
            description,
            starred,
            showcase,
            repository,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
