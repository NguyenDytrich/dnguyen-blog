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

pub mod db {

    use std::error::Error;
    use tokio_postgres::NoTls;

    /// Open a connection to Postgres on a new task
    pub async fn spawn_connection(url: &str) -> Result<tokio_postgres::Client, Box<dyn Error>> {
        let (client, connection) = tokio_postgres::connect(url, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });
        return Ok(client);
    }
}

pub mod posts {

    use std::env;
    use std::error;
    use std::vec::Vec;
    use std::convert::TryFrom;

    use chrono::prelude::*;
    use tokio_postgres::row::Row;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;


    // TODO break into struct compsition
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

    impl TryFrom<&Row> for BlogPost {
        type Error = &'static str;
        fn try_from(row: &Row) -> Result<Self, Self::Error> {
            let post = BlogPost {
                uuid: row.get::<&str, Uuid>("id"),
                created_at: row.get::<&str, DateTime<Utc>>("created_at"),
                updated_at: row.get::<&str, Option<DateTime<Utc>>>("updated_at"),
                published_at: row.get::<&str, Option<DateTime<Utc>>>("published_at"),
                is_public: row.get("is_public"),
                delta: row.get::<&str, Option<serde_json::Value>>("delta"),
                title: row.get("title")
            };
            return Ok(post);
        }
    }

    /// Retrieves a number of recent posts
    pub async fn retrieve_recent(num: i64) -> Result<Vec<BlogPost>, Box<dyn error::Error>> {
        let client = crate::db::spawn_connection(&env::var("DB_URL")?).await?;
        
        // If there are no rows, this will be an empty Vec
        let rows = client
            .query("SELECT * FROM blog_posts WHERE is_public = TRUE ORDER BY created_at DESC LIMIT $1::BIGINT", &[&num])
            .await?;

        let mut result: Vec<BlogPost> = Vec::new();

        // Cast into the BlogPost struct
        for row in rows.iter() {
            let post = BlogPost::try_from(row).unwrap();
            result.push(post);
        }

        return Ok(result);
    }

    /// Retrieve a specific post
    pub async fn retrieve_by_uuid(uuid: Uuid) -> Result<BlogPost, Box<dyn error::Error>> {
        let client = crate::db::spawn_connection(&env::var("DB_URL")?).await?;
        let row = client.query_one("SELECT * FROM blog_posts WHERE uuid=$1 AND is_public = TRUE", &[&uuid]).await?;
        let post = BlogPost::try_from(&row)?;
        return Ok(post);
    }

    /// Persist a BlogPost to the DB
    pub async fn create_draft(title: &String, delta: &Option<serde_json::Value>) -> Result<BlogPost, Box<dyn error::Error>> {
        let client = crate::db::spawn_connection(&env::var("DB_URL")?).await?;
        let row = client.query_one("INSERT INTO blog_posts (title, delta, is_public) VALUES ($1, $2, false) RETURNING *", &[&title, &delta]).await?;
        let post = BlogPost::try_from(&row).unwrap();
        return Ok(post);
    }
}

pub mod user {

    use bcrypt::{BcryptError};
    use chrono::prelude::*;
    use std::{error, env};
    use uuid::Uuid;


    pub struct Credentials {
        pub email: String,
        // Plaintext password
        pub password: String
    }

    

    impl Credentials {
        /// Convenience method for bcrypt::verify. Validates a user's credentials
        /// against a hash.
        pub fn verify(&self, hash: String) -> Result<bool, BcryptError>{
            return bcrypt::verify(&self.password, &hash);
        }
    }

    pub struct User {
        pub id: Uuid,
        pub email: String,
        pub created_at: DateTime<Utc>,
        last_login: Option<DateTime<Utc>>
    }

    /// Insert a new user, returning their UUID
    pub async fn create(creds: &Credentials) -> Result<Uuid, Box<dyn error::Error>> {
        let client = crate::db::spawn_connection(&env::var("DB_URL")?).await?;
        // TODO: Validate email
        let password_hash = bcrypt::hash(&creds.password, 10)?;
        let row = client.query_one("
            INSERT INTO users (email, password_hash)
            VALUES ($1, $2)
            RETURNING id
        ", &[&creds.email, &password_hash]).await?;

        return Ok(row.get(0));
    }

    /// Validate credentials to login a user.
    pub async fn login(creds: &Credentials) -> Result<User, Box<dyn error::Error>> {
        // Open a DB connection
        let client = crate::db::spawn_connection(&env::var("DB_URL")?).await?;

        // TODO: if this method throws an error, no user exists by that email, so return
        // appropriate error
        
        // Retrieve exactly one user.
        let rows = client.query_one("SELECT * FROM users WHERE email = $1", &[&creds.email]).await?;
        
        // Check the passwords
        let hash = rows.get::<&str, String>("password_hash");
        // TODO: return appropriate error
        creds.verify(hash).expect("Email and password combination not found");

        // Now, update the last login time
        let last_login = Utc::now();
        client.execute("UPDATE USERS SET last_login = $1 WHERE email = $2", &[&last_login, &creds.email]).await?;

        // Everything's good; return the user object
        return Ok(User {
            id: rows.get::<&str, Uuid>("id"),
            email: rows.get::<&str, String>("email"),
            created_at: rows.get::<&str, DateTime<Utc>>("created_at"),
            last_login: Some(last_login)
        });
    }

    #[cfg(test)]
    mod tests {
        use crate::user::Credentials;

        #[test]
        fn it_verifies_hashes() {
            let pwd = "supercalifragilisticexpialadocious";
            let c = Credentials {
                email: "bobert@bob.com".to_string(),
                password: pwd.to_string()
            };
            let hash = bcrypt::hash(pwd, 10).expect("Error hashing password");
            let result = c.verify(hash);
            assert!(result.is_ok());
            assert_eq!(result.ok(), Some(true));
        }
    }
}

pub mod http {
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct CreatePostArgs {
        pub delta: Option<serde_json::Value>,
        pub title: String,
    }
}
