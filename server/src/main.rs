use rocket::{get, routes, catch, catchers};
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

use dnguyen_blog::model::posts::{BlogPost, retrieve_by_uuid};
use dnguyen_blog::htmlify::transcribe;
use uuid::Uuid;
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

#[get("/blog")]
fn blog_index() -> Template {
    Template::render("blog/blog_index", context! {
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

#[get("/blog/<post_id>")]
async fn blog_post(post_id: String) -> Template {
    let u = Uuid::parse_str(&post_id);
    let post: Option<BlogPost>  = match u {
        Ok(uuid) => retrieve_by_uuid(uuid)
                        .await
                        .map_or(None, |v| Some(v)),
        Err(_) => None
    };

    return match post {
        Some(v) => Template::render(
            "blog/post", context! {
                title: v.title,
                parent: "layout",
                content: transcribe(
                    // Parse the markdown to HTML
                    &v.markdown.unwrap_or(String::new())
                )
            }),
        None => Template::render(
            "error/404", context! {
                title: "404",
                parent: "layout"
            })
    }
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
                blog_post,
                support_me
            ])
        .mount("/static", FileServer::from(relative!("static")))
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .launch()
        .await;
}
