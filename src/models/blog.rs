use crate::{
    models::{tag::Tag, ModelResult, Pagination},
    utils::{format_long_date, format_short_date},
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{prelude::FromRow, query, query_as, SqlitePool};
use tokio::time::Instant;
use uuid::{fmt::Hyphenated, Uuid};

#[derive(Debug, FromRow, Serialize)]
pub(crate) struct Blog {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub slug: String,
    pub content: String,
}

struct BlogPreviewRow {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub slug: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RecentBlog {
    pub updated_at: String,
    pub title: String,
    pub slug: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct BlogPreview {
    pub created_at: String,
    pub updated_at: String,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub tags: Vec<Tag>,
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

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub(crate) async fn query_blog_previews(
        pool: &SqlitePool,
        page: u32,
        query: Option<&str>,
    ) -> ModelResult<(Vec<BlogPreview>, u128, Pagination)> {
        assert!(page > 0, "Page must be greater than 0");

        let query_time = Instant::now();
        let offset = (i64::from(page) - 1i64) * 5i64;
        let res_iter = if query.is_none() {
            query_as!(
                BlogPreviewRow,
                r#"
                SELECT
                    id AS "id: Hyphenated",
                    created_at AS "created_at: DateTime<Utc>",
                    updated_at AS "updated_at: DateTime<Utc>",
                    title,
                    slug,
                    substr(content, 1, 160) AS "description!: String"
                FROM blog
                LIMIT 5 OFFSET $1
                "#,
                offset,
            )
            .fetch_all(pool)
            .await?
        } else {
            let actual_query = query
                .unwrap()
                .split_whitespace()
                .filter(|word| !word.starts_with('#'))
                .collect::<String>();

            query_as!(
                BlogPreviewRow,
                r#"
                SELECT
                    id AS "id: Hyphenated",
                    created_at AS "created_at: DateTime<Utc>",
                    updated_at AS "updated_at: DateTime<Utc>",
                    title,
                    slug,
                    substr(content, 1, 160) AS "description!: String"
                FROM blog
                WHERE
                    lower(title) LIKE lower('%' || $1 || '%') OR
                    lower(content) LIKE lower('%' || $1 || '%')
                LIMIT 5 OFFSET $2
                "#,
                actual_query,
                offset,
            )
            .fetch_all(pool)
            .await?
        }
        .into_iter();
        let mut res = Vec::new();
        for blog in res_iter {
            res.push(BlogPreview {
                created_at: format_long_date(blog.created_at),
                updated_at: format_long_date(blog.updated_at),
                title: blog.title,
                slug: blog.slug,
                description: blog.description,
                tags: Tag::get_tags_from_blog_id(pool, blog.id).await?,
            });
        }

        let total_items = query!("SELECT count(*) AS count FROM blog")
            .fetch_one(pool)
            .await?
            .count;
        let last_page = (f64::from(total_items) / 5f64).ceil() as u64;

        Ok((
            res,
            query_time.elapsed().as_millis(),
            Pagination {
                current_page: u64::from(page),
                last_page,
                total_items: total_items as u64,
            },
        ))
    }
}
