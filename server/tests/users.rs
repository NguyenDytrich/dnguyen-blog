use dnguyen_blog::model::users;
use dnguyen_blog::model::users::{User, Credentials};
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
    let uuid: Uuid = users::create(&creds).await.unwrap();

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
    assert!(creds.verify(&hash).unwrap());
    
    // 5) Cleanup
    common::db::reset("users").await.expect("Error resetting table: users");
}

#[tokio::test]
async fn it_logs_in_users() {
    // Reset DB
    common::db::reset("users").await.expect("Error resetting table: users");
    let client = spawn_connection(&env::var("DB_URL").unwrap()).await.unwrap();

    // Login the user
    let creds = Credentials { email: SafeEmail().fake(), password: Password(10..100).fake()};
    let uuid: Uuid = users::create(&creds).await.unwrap();

    let u = users::login(&creds).await.unwrap();

    assert_eq!(uuid, u.id);
    assert_eq!(creds.email, u.email);
}

#[tokio::test]
async fn it_doesnt_log_in_invalid_credentials() {
    // Reset DB
    common::db::reset("users").await.expect("Error resetting table: users");
    let client = spawn_connection(&env::var("DB_URL").unwrap()).await.unwrap();

    // Login the user
    let mut creds = Credentials { email: SafeEmail().fake(), password: Password(10..100).fake()};
    let uuid: Uuid = users::create(&creds).await.unwrap();

    creds.password = Password(10..100).fake();
    let u = users::login(&creds).await;

    assert!(u.is_err());
}

#[tokio::test]
async fn it_signs_up_users() {
    // Reset DB
    common::db::reset("users").await.expect("Error resetting table: users");
    let client = spawn_connection(&env::var("DB_URL").unwrap()).await.unwrap();

    let email: String = SafeEmail().fake();
    let password: String  = Password(10..100).fake();
    let password_conf: String = password.to_owned();

    // Signup the user
    let u = users::signup(&email, &password, &password_conf).await.unwrap();

    // Email should be the same
    assert_eq!(email, u.email);

    // Now we should be able to login with the credentials
    let creds = Credentials { email: email.clone(), password: password.clone() };
    let l = users::login(&creds).await.unwrap(); 

    assert_eq!(u.id, l.id);
    assert_eq!(email, l.email);
}
