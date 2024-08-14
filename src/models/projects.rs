use chrono::{DateTime, Utc};
use sqlx::{prelude::FromRow, query, SqlitePool};
use uuid::Uuid;

use super::{ModelResult, Repository};

#[derive(FromRow)]
pub(crate) struct Project {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub repository: String,
}

// TODO: Very unfinished
impl Project {
    async fn create(
        pool: &SqlitePool,
        name: &str,
        description: &str,
        repository: &str,
    ) -> ModelResult<()> {
        let id = Uuid::new_v4();
        query!(
            "INSERT INTO projects (id, name, description, repository) VALUES ($1, $2, $3, $4)",
            id,
            name,
            description,
            repository,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
