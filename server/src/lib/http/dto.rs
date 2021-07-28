use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreatePostArgs {
    pub delta: Option<serde_json::Value>,
    pub title: String,
}
