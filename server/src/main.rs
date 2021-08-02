use rocket::{get, routes};
use rocket::fs::{FileServer, relative};
use dotenv::dotenv;

use rocket_dyn_templates::Template;

mod routes;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        title: "Home",
        parent: "layout"
    })
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
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .launch()
        .await;
}
