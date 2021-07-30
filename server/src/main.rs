use rocket::{get, post, routes};
use rocket::serde::json::Json;
use rocket::response::status;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};

use dnguyen_blog::model::posts;
use dnguyen_blog::model::users;
use dnguyen_blog::model::users::{User, Credentials};
use dnguyen_blog::http::dto::{CreatePostArgs, SignupArgs};
use dotenv::dotenv;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// By default, retrieve 10 most recent posts
#[get("/posts")]
async fn recent_posts() -> Option<String> {
    let post = match posts::retrieve_recent(10).await {
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
    let post = match posts::retrieve_recent(count).await {
        Ok(r) => match serde_json::to_string(&r) {
            Ok(v) => Some(v),
            Err(_) => None,
        },
        Err(_) => None,
    };

    return post;
}

/// Create a new post with arguments from posted JSON
#[post("/posts/draft", format = "json", data = "<post_args>")]
async fn new_post(user: User, post_args: Json<CreatePostArgs>) -> status::Accepted<()> {
    println!("user {} posted a draft.", user.id);
    posts::create_draft(&post_args.title, &post_args.delta).await.unwrap();
    return status::Accepted(Some(()));
}

#[post("/login", data = "<credentials>")]
async fn login(cookies: &CookieJar<'_>, credentials: Form<Credentials>) -> status::Accepted<()> {
    // TODO return meaningful error
    let user = users::login(&credentials).await.unwrap();

    // Set a private cookie
    // TODO this could be a session ID
    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
    return status::Accepted(Some(()));
}

#[post("/signup", data = "<signup>")]
async fn signup(signup: Form<SignupArgs>) -> status::Accepted<()> {
    users::signup(
        &signup.email, 
        &signup.password, 
        &signup.password_conf).await.unwrap();

    return status::Accepted(Some(()));
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    let _server = rocket::build()
        .mount("/", routes![index, recent_posts, recent_posts_count, new_post, login, signup])
        .launch()
        .await;
}
