use super::ModelResult;
use chrono::{DateTime, Datelike, Utc};
use serde::Serialize;
use sqlx::{prelude::FromRow, query, SqlitePool};
use uuid::Uuid;

#[derive(FromRow, Serialize)]
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
        let res = query!(
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
        .await?;
        let res = res
            .into_iter()
            .map(|blog| RecentBlog {
                updated_at: Self::format_date(blog.updated_at),
                title: blog.title,
                slug: blog.slug,
            })
            .collect();

        Ok(res)
    }

    fn format_date(date: DateTime<Utc>) -> String {
        let day = match date.day() {
            day @ (1 | 21 | 31) => format!("{day:02}st"),
            day @ (2 | 22) => format!("{day:02}nd"),
            day @ (3 | 23) => format!("{day:02}rd"),
            day @ (1..=31) => format!("{day:02}th"),
            _ => unreachable!(),
        };
        format!("{} {}", day, date.format("%b. %Y"))
    }
}
