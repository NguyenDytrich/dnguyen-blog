pub mod error {

    use std::error::Error;
    use std::fmt;

    use rocket::request::Request;
    use rocket::response::{self, Responder, Response};
    use rocket::http::Status;

    #[derive(Debug)]
    /// Wrapper class for tokio_postgres errors
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

pub mod posts {

    use std::env;
    use std::error;
    use std::vec::Vec;

    use chrono::prelude::*;
    use tokio;
    use tokio_postgres::{NoTls};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;


    /// Representation of the BlogPost
    #[derive(Serialize, Deserialize)]
    pub struct BlogPost {
        pub uuid: Uuid,
        pub created_at: DateTime<Utc>,
        pub updated_at: Option<DateTime<Utc>>,
        pub published_at: Option<DateTime<Utc>>,
        pub is_public: bool,
        pub delta: Option<serde_json::Value>,
        pub title: String
    }

    /// Retrieves a number of recent posts
    pub async fn retrieve_recent(num: i64) -> Result<Vec<BlogPost>, Box<dyn error::Error>> {
        let (client, connection) = tokio_postgres::connect(&env::var("DB_URL")?, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        // If there are no rows, this will be an empty Vec
        let rows = client
            .query("SELECT * FROM blog_posts ORDER BY created_at DESC LIMIT $1::BIGINT", &[&num])
            .await?;

        let mut result: Vec<BlogPost> = Vec::new();

        // Cast into the BlogPost struct
        for row in rows.iter() {
            let post = BlogPost {
                uuid: row.get::<&str, Uuid>("id"),
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
