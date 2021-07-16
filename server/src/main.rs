use rocket::{get, routes};
use dnguyen_blog::posts::{retrieve_recent};
use dotenv::dotenv;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// By default, retrieve 10 most recent posts
#[get("/posts")]
async fn recent_posts() -> Option<String> {
    let post = match retrieve_recent(10).await {
        Ok(r) => match serde_json::to_string(&r) {
            Ok(v) => Some(v),
            Err(_) => None,
        },
        Err(_) => None,
    };

    return post;
}

/// Retreives a number of recent posts in JSON format
#[get("/posts?<count>")]
async fn recent_posts_count(count: i64) -> Option<String> {
    let post = match retrieve_recent(count).await {
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
    dotenv().ok();

    let _server = rocket::build()
        .mount("/", routes![index, recent_posts, recent_posts_count])
        .launch()
        .await;
}

