use rocket::{get, routes, catch, catchers};
use rocket::fs::{FileServer, relative};
use dotenv::dotenv;

use rocket_dyn_templates::Template;

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

#[get("/blog")]
fn blog_index() -> Template {
    Template::render("blog_index", context! {
        title: "Blog",
        parent: "layout",
        blog_posts: [
            context! {
                title:  "Test post",
                date: "09 August, 2021",
                preview: "Lorem ipsum"
            }
        ],
        paginate: context! {
            prev: false,
            next: true,
            current: 1,
            total: 10
        }
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
                blog_index,
                support_me
            ])
        .mount("/static", FileServer::from(relative!("static")))
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .launch()
        .await;
}
