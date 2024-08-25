use crate::{models::ModelResult, utils::format_short_date};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{prelude::FromRow, query, SqlitePool};
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub(crate) struct Blog {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub slug: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RecentBlog {
    pub updated_at: String,
    pub title: String,
    pub slug: String,
}

impl Blog {
    pub(crate) async fn get_recent_blogs(pool: &SqlitePool) -> ModelResult<Vec<RecentBlog>> {
        Ok(query!(
            r#"
            SELECT
                updated_at AS "updated_at: DateTime<Utc>",
                title,
                slug
            FROM blog
            ORDER BY created_at DESC LIMIT 3
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|blog| RecentBlog {
            updated_at: format_short_date(blog.updated_at),
            title: blog.title,
            slug: blog.slug,
        })
        .collect())
    }
}
