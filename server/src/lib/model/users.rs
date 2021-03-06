use bcrypt::{BcryptError};
use chrono::prelude::*;
use std::{error, env};
use uuid::Uuid;

use rocket::outcome::Outcome;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::form::FromForm;

#[derive(FromForm)]
pub struct Credentials {
    pub email: String,
    // Plaintext password
    pub password: String
}

// TODO meaningful errors
#[derive(Debug)]
pub struct SignupError;
impl SignupError {
    fn new() -> SignupError {
        SignupError
    }
}
impl error::Error for SignupError {
    fn description(&self) -> &str { "Signup error" }
}
impl std::fmt::Display for SignupError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "Signup error")
    }
}

#[derive(Debug)]
pub struct LoginError {
    description: String
}
impl LoginError {
    fn new(d: &str) -> LoginError {
        LoginError {
            description: d.to_string()
        }
    }
}
impl error::Error for LoginError {
    fn description(&self) -> &str { &self.description }
}
impl std::fmt::Display for LoginError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Credentials {
    /// Convenience method for bcrypt::verify. Validates a user's credentials
    /// against a hash.
    pub fn verify(&self, hash: &str) -> Result<bool, BcryptError>{
        return bcrypt::verify(&self.password, hash);
    }
}

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    last_login: Option<DateTime<Utc>>
}

// TODO move to http module. This code is Guard code.
#[derive(Debug)]
pub enum UserError {
    DoesNotExist,
    Suspended,
    Unauthorized
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = UserError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        // Get the cookie jar
        let cookie = request.cookies()
            // Read the private user_id cookie
            .get_private("user_id")
            // Parse the value of said cookie
            .map(|cookie| Uuid::parse_str(cookie.value()));

        // If there is no cookie, return 400 with Unauthorized error.
        let cookie = match cookie {
            Some(v) => v,
            None => return Outcome::Failure((Status::Unauthorized, UserError::Unauthorized))
        };

        // Early return if the value couldn't be parsed as a UUID
        let uid: Uuid = match cookie {
            Ok(r) => r,
            Err(_) => return Outcome::Failure((Status::Unauthorized, UserError::Unauthorized))
        };

        // TODO Cache this value
        // If our user exists in the DB, succeed. Otherwise, the user does not exist.
        let user = retrieve_by_uuid(&uid).await;
        match user {
            Ok(u) => Outcome::Success(u),
            Err(_) => Outcome::Failure((Status::Unauthorized, UserError::DoesNotExist)),
        }

    }
}
// End TODO

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

pub async fn retrieve_by_uuid(uuid: &Uuid) -> Result<User, Box<dyn error::Error>> {
    let client = crate::db::spawn_connection(&env::var("DB_URL")?).await?;
    let row = client.query_one("SELECT * FROM users WHERE id = $1::UUID", &[&uuid]).await?;

    return Ok(User {
        id: row.get::<&str, Uuid>("id"),
        email: row.get::<&str, String>("email"),
        created_at: row.get::<&str, DateTime<Utc>>("created_at"),
        last_login: row.get::<&str, Option<DateTime<Utc>>>("last_login")
    });
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
    let is_valid = creds.verify(&hash)?;

    // TODO don't panic
    if !is_valid {
        return Err(LoginError::new("No user with the given email/password combination").into());
    }

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

pub async fn signup(email: &str, password: &str, password_conf: &str) -> Result<User, Box<dyn error::Error>> {
    if password != password_conf {
        // TODO meaningful errors
        return Err(SignupError::new().into());
    }
    // TODO Additional validation here

    let client = crate::db::spawn_connection(&env::var("DB_URL")?).await?;
    // TODO config hash cost
    let hash = bcrypt::hash(password, 10)?;

    let rows = client.query_one(
        "INSERT INTO users (email, password_hash)
        VALUES ($1, $2)
        RETURNING id, email, created_at", &[&email, &hash]).await?;

    return Ok(User {
        id: rows.get::<&str, Uuid>("id"),
        email: rows.get::<&str, String>("email"),
        created_at: rows.get::<&str, DateTime<Utc>>("created_at"),
        last_login: None
    });
}

#[cfg(test)]
mod tests {
    use crate::model::users::Credentials;

    #[test]
    fn it_verifies_hashes() {
        let pwd = "supercalifragilisticexpialadocious";
        let c = Credentials {
            email: "bobert@bob.com".to_string(),
            password: pwd.to_string()
        };
        let hash = bcrypt::hash(pwd, 10).expect("Error hashing password");
        let result = c.verify(&hash);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(true));
    }
}
