use rocket::{get, post, routes};
use rocket::response::status;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};

use dnguyen_blog::model::users;
use dnguyen_blog::model::users::Credentials;
use dnguyen_blog::http::dto::SignupArgs;
use dotenv::dotenv;

mod routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
        .mount("/api", routes![
                routes::api::blog_posts::recent,
                routes::api::blog_posts::recent_count,
                routes::api::blog_posts::new
            ])
        .mount("/", routes![index, login, signup])
        .launch()
        .await;
}
