use rocket::{get, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .mount("/test", routes![db_test])
        .launch()
        .await;
}