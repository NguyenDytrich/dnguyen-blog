mod dnguyen;

use rocket::{get, routes};
use dnguyen::blog::{retrieve_first_post};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/db_test")]
async fn db_test() -> Option<String> {
    return match retrieve_first_post().await {
        Ok(r) => Some(r.to_json().to_string()),
        Err(_) => None,
    };
}

#[rocket::main]
async fn main() {
    let _server = rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![db_test])
        .launch()
        .await;
}

