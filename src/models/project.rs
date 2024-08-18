use super::ModelResult;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{prelude::FromRow, query, query_as, SqlitePool};
use uuid::{fmt::Hyphenated, Uuid};

#[derive(Debug, FromRow, Serialize)]
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
    #[allow(dead_code)]
    pub(crate) async fn create(
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

    pub(crate) async fn get_all_projects(pool: &SqlitePool) -> ModelResult<Vec<Self>> {
        query_as!(
            Self,
            r#"
            SELECT id AS "id: Hyphenated",
                created_at AS "created_at: DateTime<Utc>",
                updated_at AS "updated_at: DateTime<Utc>",
                name,
                description,
                starred AS "starred: bool",
                showcase,
                repository
            FROM project
            "#
        )
        .fetch_all(pool)
        .await
    }
}
