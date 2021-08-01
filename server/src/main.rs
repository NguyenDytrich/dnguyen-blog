use rocket::{get, routes};
use dotenv::dotenv;

mod routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}



#[rocket::main]
async fn main() {
    dotenv().ok();

    let _server = rocket::build()
        .mount("/api/v1", routes![
                routes::api::blog_posts::recent,
                routes::api::blog_posts::recent_count,
                routes::api::blog_posts::new
            ])
        .mount("/auth", routes![
                routes::api::auth::login,
                routes::api::auth::signup,
            ])
        .mount("/", routes![index])
        .launch()
        .await;
}
