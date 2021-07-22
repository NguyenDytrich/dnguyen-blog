use dnguyen_blog::user;
use dnguyen_blog::user::{User, Credentials};
use dnguyen_blog::db::spawn_connection;

use std::env;
use tokio_postgres::row::Row;
use uuid::Uuid;
use fake::{Fake};
use fake::faker::internet::en::{SafeEmail, Password};

mod common;

#[tokio::test]
async fn it_hashes_passwords() {
    // 1) Reset DB
    common::db::reset("users").await.expect("Error resetting table: users");

    // 2) Call a user create method with credentials
    let creds = Credentials { email: SafeEmail().fake(), password: Password(10..100).fake()};
    let uuid: Uuid = user::create(&creds).await.unwrap();

    // 3) Verify that the password_hash field in the SQL table is not equal to plaintext
    let client = spawn_connection(&env::var("DB_URL").unwrap()).await.unwrap();
    let row = client.query_one("SELECT * FROM users LIMIT 1", &[]).await.unwrap();
    let u = row.get::<&str, Uuid>("id");
    let e = row.get::<&str, String>("email");
    let hash = row.get::<&str, String>("password_hash");

    assert_eq!(u, uuid);
    assert_eq!(e, creds.email);
    assert_ne!(creds.password, hash);

    // 4) Verify that the password matches the password_hash field when comparing using BCrypt
    assert!(creds.verify(hash).unwrap());
    
    // 5) Cleanup
    common::db::reset("users").await.expect("Error resetting table: users");
}
