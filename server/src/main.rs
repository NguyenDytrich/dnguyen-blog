use rocket::{get, routes, catch, catchers};
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

use dotenv::dotenv;

mod routes;

#[get("/")]
fn index() -> Template {
    Template::render("about", ())
}

#[get("/support")]
fn support_me() -> Template {
    Template::render("support", context! {
        title: "Support Me",
        parent: "layout"
    })
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
        .mount("/", routes![
                index,
                routes::blog::blog_index,
                routes::blog::blog_post,
                routes::blog::blog,
                support_me
            ])
        .mount("/static", FileServer::from(relative!("static")))
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .launch()
        .await;
}
