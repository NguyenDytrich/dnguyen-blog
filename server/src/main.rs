use rocket::{get, routes, catch, catchers};
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

use dnguyen_blog::model::posts;
use dnguyen_blog::model::posts::{BlogPost, retrieve_by_uuid, retrieve_recent, get_post_count};
use dnguyen_blog::htmlify::{transcribe, monthify};
use dnguyen_blog::http::dto::BlogPostPreview;
use std::vec;
use chrono::prelude::*;
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
async fn blog_index() -> Template {
    let num_retrieved = 5;
    // Retrieve posts or return empty vector
    let posts: Vec<BlogPost> = retrieve_recent(num_retrieved)
        .await
        .unwrap_or(Vec::new());

    // Cast to the data object
    let mut mapped_posts: Vec<BlogPostPreview> = Vec::new();
    for p in posts.iter() {
        let date = match p.published_at {
            Some(d) => (d.day(), d.month(), d.year()),
            None => (p.created_at.day(), p.created_at.month(), p.created_at.year())
        };
        let preview = BlogPostPreview {
            uuid_repr: p.uuid.to_string(),
            title: p.title.to_owned(),
            date_repr: format!("{:02}, {} {}", 
                date.0, 
                monthify(date.1 as usize).unwrap_or("ERR".to_string()),
                date.2),
            preview: p.markdown
                .to_owned()
                .unwrap_or(String::new())
        };
        mapped_posts.insert(0, preview);
    }

    // Calculate pagination
    let count = get_post_count().await.unwrap_or(0);
    let pages = count as i64 / num_retrieved;
    let next = pages > 1;

    Template::render("blog/blog_index", context! {
        title: "Blog",
        parent: "layout",
        blog_posts: mapped_posts,
        paginate: context! {
            prev: false,
            next: next,
            current: 1,
            prev_page: 0,
            next_page: 2,
            total: pages
        }
    })
}

#[get("/blog?<page>")]
async fn blog(page: usize) -> Template {
    let num_retrieved = 5;

    let posts: Vec<BlogPost> = posts::retrieve_with_offset(num_retrieved, num_retrieved * (page - 1) as i64)
        .await
        .unwrap_or(Vec::new());

    let mut mapped_posts: Vec<BlogPostPreview> = Vec::new();
    for p in posts.iter() {
        let date = match p.published_at {
            Some(d) => (d.day(), d.month(), d.year()),
            None => (p.created_at.day(), p.created_at.month(), p.created_at.year())
        };
        let preview = BlogPostPreview {
            uuid_repr: p.uuid.to_string(),
            title: p.title.to_owned(),
            date_repr: format!("{:02}, {} {}", 
                date.0, 
                monthify(date.1 as usize).unwrap_or("ERR".to_string()),
                date.2),
            preview: p.markdown
                .to_owned()
                .unwrap_or(String::new())
        };
        mapped_posts.insert(0, preview);
    }

    // Calculate pagination
    let count = get_post_count().await.unwrap_or(0);
    let pages = count as i64 / num_retrieved;
    let next = pages > page as i64;
    let prev = page > 1;

    Template::render("blog/blog_index", context! {
        title: "Blog",
        parent: "layout",
        blog_posts: mapped_posts,
        paginate: context! {
            prev: prev,
            next: next,
            current: page,
            next_page: page + 1,
            prev_page: page - 1,
            total: pages
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
                blog,
                support_me
            ])
        .mount("/static", FileServer::from(relative!("static")))
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .launch()
        .await;
}
