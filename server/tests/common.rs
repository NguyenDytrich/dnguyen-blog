pub mod db {

    use std::env;
    use std::error;

    use uuid::Uuid;

    use dnguyen_blog::db::spawn_connection;

    use fake::{Fake};
    use fake::faker::lorem::en::Word;

    pub fn setup() {
        // TODO: empty db
    }

    /// Resets a table by name. WARNING: you can inject SQL with this function
    pub async fn reset(table_name: &str) -> Result<(), Box<dyn error::Error>> {
        let client = spawn_connection(&env::var("DB_URL")?).await?;
        let statement = format!("DELETE FROM {}", table_name);
        let rows = client
            .execute(&statement[..], &[]).await?;
        return Ok(());
    }

    pub fn teardown() {
    }

    /// Create a post, then return its ID
    pub async fn create_random_post() -> Result<Uuid, Box<dyn error::Error>> {
        let client = spawn_connection(&env::var("DB_URL")?).await?;
        let fake_title: String = Word().fake();

        let rows = client
            .query("
                INSERT INTO blog_posts (published_at, is_public, title) 
                VALUES (CURRENT_TIMESTAMP, TRUE, $1)
                RETURNING id", &[&fake_title]).await?;

        return Ok(rows[0].get(0));
    }

    pub async fn create_unpublished_post() -> Result<Uuid, Box<dyn error::Error>> {
        let client = spawn_connection(&env::var("DB_URL")?).await?;
        let fake_title: String = Word().fake();

        let rows = client
            .query("
                INSERT INTO blog_posts (published_at, is_public, title) 
                VALUES (CURRENT_TIMESTAMP, FALSE, $1)
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
        let client = spawn_connection(&env::var("DB_URL")?).await?;
        let row = client.query_one("SELECT * FROM blog_posts LIMIT 1", &[]).await?;
        return Ok(row);
    }
}
