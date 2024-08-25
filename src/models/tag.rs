use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{query_as, FromRow, SqlitePool};
use uuid::{fmt::Hyphenated, Uuid};

use super::ModelResult;

#[derive(Debug, FromRow, Serialize)]
pub(crate) struct Tag {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
}

impl Tag {
    pub(crate) async fn get_tags_from_blog_id(
        pool: &SqlitePool,
        blog_id: Uuid,
    ) -> ModelResult<Vec<Self>> {
        query_as!(
            Self,
            r#"
            SELECT
                tag.id AS "id: Hyphenated",
                tag.created_at AS "created_at: DateTime<Utc>",
                tag.updated_at AS "updated_at: DateTime<Utc>",
                tag.name
            FROM tag
            JOIN blog_tag ON blog_tag.tag_id = tag.id
            WHERE blog_tag.blog_id = $1
            "#,
            *blog_id.as_hyphenated(),
        )
        .fetch_all(pool)
        .await
    }
}
