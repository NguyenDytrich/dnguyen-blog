pub mod db {

    use std::env;
    use std::error;

    use tokio;
    use tokio_postgres::{NoTls};
    use uuid::Uuid;

    use fake::{Fake};
    use fake::faker::lorem::en::Word;

    pub fn setup() {
        // TODO: empty db
    }

    pub async fn reset(table_name: &str) -> Result<(), Box<dyn error::Error>> {
        //TODO create a singleton connection pool?
        //TODO change to env var
        let (client, connection) = tokio_postgres::connect(&env::var("DB_URL")?, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        let rows = client
            .execute("DELETE FROM blog_posts", &[]).await?;
        return Ok(());
    }

    pub fn teardown() {
    }

    /// Create a post, then return its ID
    pub async fn create_random_post() -> Result<Uuid, Box<dyn error::Error>> {
        //TODO create a singleton connection pool?
        //TODO change to env var
        let (client, connection) = tokio_postgres::connect(&env::var("DB_URL")?, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        let fake_title: String = Word().fake();

        let rows = client
            .query("
                INSERT INTO blog_posts (published_at, is_public, title) 
                VALUES (CURRENT_TIMESTAMP, TRUE, $1)
                RETURNING id", &[&fake_title]).await?;

        return Ok(rows[0].get(0));
    }

    /// Create multiple posts, returning a Vec<Uuid> of their uuids
    pub async fn create_random_posts(num: u32) -> Result<Vec<Uuid>, Box<dyn error::Error>> {
        let mut result: Vec<Uuid> = Vec::new();

        for _i in 0..num {
            result.push(create_random_post().await?);
        }

        return Ok(result);
    }

    pub async fn get_first_post() -> Result<tokio_postgres::Row, Box<dyn error::Error>> {
        //TODO create a singleton connection pool?
        //TODO change to env var
        let (client, connection) = tokio_postgres::connect(&env::var("DB_URL")?, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });
        let row = client.query_one("SELECT * FROM blog_posts LIMIT 1", &[]).await?;
        return Ok(row);
    }
}
