pub mod error {

    // Wrapper class for tokio_postgres

    use std::error::Error;
    use std::fmt;

    use rocket::request::Request;
    use rocket::response::{self, Responder, Response};
    use rocket::http::Status;

    #[derive(Debug)]
    pub struct DBError;

    impl fmt::Display for DBError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "DBError.")
        }
    }
    impl Error for DBError {}
    impl<'r> Responder<'r, 'r> for DBError {
        fn respond_to(self, _: &Request) -> response::Result<'r> {
            return Response::build()
                .status(Status::BadRequest)
                .ok();
        }
    }
    impl From<tokio_postgres::Error> for DBError {
        fn from(_: tokio_postgres::Error) -> Self {
            return DBError {};
        }
    }
}

pub mod blog {

    use std::env;
    use std::error;
    use std::vec::Vec;

    use chrono::prelude::*;

    use tokio;
    use tokio_postgres::{NoTls};

    use serde::{Deserialize, Serialize};


    /// Representation of the BlogPost
    #[derive(Serialize, Deserialize)]
    pub struct BlogPost {
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
        published_at: Option<DateTime<Utc>>,
        is_public: bool,
        delta: Option<serde_json::Value>,
        title: String
    }

    /// Retrieves the 10 most recent posts
    pub async fn retrieve_recent_posts() -> Result<Vec<BlogPost>, Box<dyn error::Error>> {
        let (client, connection) = tokio_postgres::connect(&env::var("DB_URL")?, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        let rows = client
            .query("SELECT * FROM blog_posts ORDER BY created_at DESC LIMIT 10", &[])
            .await?;

        let mut result: Vec<BlogPost> = Vec::new();

        for row in rows.iter() {
            let post = BlogPost {
                created_at: row.get::<&str, DateTime<Utc>>("created_at"),
                updated_at: row.get::<&str, Option<DateTime<Utc>>>("updated_at"),
                published_at: row.get::<&str, Option<DateTime<Utc>>>("published_at"),
                is_public: row.get("is_public"),
                delta: row.get::<&str, Option<serde_json::Value>>("delta"),
                title: row.get("title")
            };

            result.push(post);
        }

        return Ok(result);
    }
}
