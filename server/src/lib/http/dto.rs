use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreatePostArgs {
    pub delta: Option<serde_json::Value>,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpArgs {
    pub email: String,
    pub password: String,
    pub password_conf: String
}
