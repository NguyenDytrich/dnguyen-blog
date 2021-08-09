use rocket::{get, routes, catch, catchers};
use rocket::fs::{FileServer, relative};
use dotenv::dotenv;

use rocket_dyn_templates::Template;

mod routes;

#[get("/")]
fn index() -> Template {
    Template::render("about", ())
}

#[catch(404)]
fn not_found() -> Template {
    Template::render("error/404", context! {
        title: "404",
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
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .launch()
        .await;
}
