pub mod error {

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

    use chrono::prelude::*;
    use std::fmt;
    use std::error;

    use tokio;
    use tokio_postgres::{NoTls};

    use serde_json::json;


    pub struct BlogPost {
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
        published_at: Option<DateTime<Utc>>,
        is_public: bool,
        delta: Option<serde_json::Value>,
        title: String
    }

    impl BlogPost {
        pub fn to_json(&self) -> serde_json::Value {
            let optional_date = |d: Option<DateTime<Utc>>| {
                return match d {
                    Some(s) => Some(s.to_string()),
                    None => None,
                }
            };

            let post = json!({
                "createdAt": self.created_at.to_string(),
                "updatedAt": optional_date(self.updated_at),
                "publishedAt": optional_date(self.published_at),
                "is_public": self.is_public,
                "delta": self.delta,
                "title": self.title,
            });

            return post
        }
    }

    
    impl fmt::Display for BlogPost {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "\"{}\" (created {})", self.title, self.created_at)
        }
    }

    pub async fn retrieve_first_post() -> Result<BlogPost, Box<dyn error::Error>> {
        let (client, connection) = tokio_postgres::connect("host=localhost user=dytrich", NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        let row = client
            .query("SELECT * FROM blog_posts ORDER BY created_at DESC LIMIT 1", &[])
            .await?;

        let post = BlogPost {
            created_at: row[0].get::<&str, DateTime<Utc>>("created_at"),
            updated_at: row[0].get::<&str, Option<DateTime<Utc>>>("updated_at"),
            published_at: row[0].get::<&str, Option<DateTime<Utc>>>("published_at"),
            is_public: row[0].get("is_public"),
            delta: row[0].get::<&str, Option<serde_json::Value>>("delta"),
            title: row[0].get("title")
        };

        return Ok(post);
    }
}
