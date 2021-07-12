mod dnguyen;

use rocket::{get, routes};
use dnguyen::blog::{retrieve_recent_posts};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// Retreives the 10 most recent posts in JSON format
#[get("/recent")]
async fn recent_posts() -> Option<String> {
    let post = match retrieve_recent_posts().await {
        Ok(r) => match serde_json::to_string(&r) {
            Ok(v) => Some(v),
            Err(_) => None,
        },
        Err(_) => None,
    };

    return post;
}

#[rocket::main]
async fn main() {
    let _server = rocket::build()
        .mount("/", routes![index])
        .mount("/posts", routes![recent_posts])
        .launch()
        .await;
}

