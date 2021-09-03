use std::env;
use std::error;
use std::vec::Vec;
use std::convert::TryFrom;

use chrono::prelude::*;
use tokio_postgres::row::Row;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::http::dto::CreatePostArgs;

// TODO break into struct compsition
/// Representation of the BlogPost
#[derive(Serialize, Deserialize)]
pub struct BlogPost {
    pub uuid: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub is_public: bool,
    pub markdown: Option<String>,
    pub title: String
}

impl TryFrom<&Row> for BlogPost {
    type Error = &'static str;
    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let post = BlogPost {
            uuid: row.get::<&str, Uuid>("id"),
            created_at: row.get::<&str, DateTime<Utc>>("created_at"),
            updated_at: row.get::<&str, Option<DateTime<Utc>>>("updated_at"),
            published_at: row.get::<&str, Option<DateTime<Utc>>>("published_at"),
            is_public: row.get("is_public"),
            markdown: row.get::<&str, Option<String>>("markdown"),
            title: row.get("title")
        };
        return Ok(post);
    }
}

/// Retrieves a number of recent posts
pub async fn retrieve_recent(num: i64) -> Result<Vec<BlogPost>, Box<dyn error::Error>> {
    retrieve_with_offset(num, 0).await
} 

/// Retrieves a number of posts, in descending order by date, offset by a number of posts
pub async fn retrieve_with_offset(num: i64, offset: i64) -> Result<Vec<BlogPost>, Box<dyn error::Error>> {
    let client = crate::db::spawn_connection(&env::var("DB_URL")?).await?; let rows = client
        .query("
            SELECT * FROM
            blog_posts WHERE
            is_public = TRUE
            ORDER BY created_at DESC
            LIMIT $1::BIGINT OFFSET $2::BIGINT
        ", &[&num, &offset])
        .await?;

    let mut result: Vec<BlogPost> = Vec::new();

    for row in rows.iter() {
        let post = BlogPost::try_from(row).unwrap();
        result.push(post);
    }

    Ok(result)
}

/// Retrieve a specific post
pub async fn retrieve_by_uuid(uuid: Uuid) -> Result<BlogPost, Box<dyn error::Error>> {
    let client = crate::db::spawn_connection(&env::var("DB_URL")?).await?;
    let row = client.query_one("SELECT * FROM blog_posts WHERE id=$1 AND is_public = TRUE", &[&uuid]).await?;
    let post = BlogPost::try_from(&row)?;
    return Ok(post);
}

/// Persist a BlogPost to the DB
pub async fn create(args: CreatePostArgs) -> Result<BlogPost, Box<dyn error::Error>> {
    let client = crate::db::spawn_connection(&env::var("DB_URL")?).await?;
    let published = match args.is_public {
        Some(v) => v,
        None => false
    };

    let row = client.query_one("
        INSERT INTO blog_posts (title, markdown, is_public) 
        VALUES ($1, $2, $3) RETURNING *", &[&args.title, &args.markdown, &published]).await?;

    let post = BlogPost::try_from(&row).unwrap();
    return Ok(post);
}
