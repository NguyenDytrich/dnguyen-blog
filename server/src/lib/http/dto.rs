use serde::{Serialize, Deserialize};
use rocket::form::FromForm;

#[derive(Serialize, Deserialize)]
pub struct CreatePostArgs {
    pub delta: Option<serde_json::Value>,
    pub title: String,
}

#[derive(FromForm)]
pub struct SignupArgs {
    pub email: String,
    pub password: String,
    pub password_conf: String
}
