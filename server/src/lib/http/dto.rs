use serde::{Serialize, Deserialize};
use rocket::form::FromForm;

#[derive(Serialize, Deserialize)]
pub struct CreatePostArgs {
    pub markdown: Option<String>,
    pub is_public: Option<bool>,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePostArgs {
    pub markdown: Option<String>,
    pub is_public: Option<bool>,
    pub title: Option<String>,
}

#[derive(FromForm)]
pub struct SignupArgs {
    pub email: String,
    pub password: String,
    pub password_conf: String
}
