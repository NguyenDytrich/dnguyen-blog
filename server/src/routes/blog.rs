use rocket::get;
use rocket_dyn_templates::Template;

use dnguyen_blog::model::posts;
use dnguyen_blog::model::posts::BlogPost;
use dnguyen_blog::htmlify::{transcribe, monthify};
use dnguyen_blog::http::dto::BlogPostPreview;

use chrono::prelude::*;
use uuid::Uuid;

async fn aggregate_blog_posts(count: i64, offset: i64) -> Vec<BlogPostPreview> {
    let posts: Vec<BlogPost> = posts::retrieve_with_offset(count, offset)
        .await
        .unwrap_or(Vec::new());

    // Cast to the data object
    let mut mapped_posts: Vec<BlogPostPreview> = Vec::new();
    for p in posts.iter() {
        let date = match p.published_at {
            Some(d) => (d.day(), d.month(), d.year()),
            None => (p.created_at.day(), p.created_at.month(), p.created_at.year())
        };
        let markdown = p.markdown.to_owned().unwrap_or(String::new());
        let mut preview = String::new();
        if markdown.len() > 300 {
            preview.push_str(&markdown[1..=255]);
            preview.push_str("...");
        } else {
            preview = markdown;
        }

        let post = BlogPostPreview {
            uuid_repr: p.uuid.to_string(),
            title: p.title.to_owned(),
            date_repr: format!("{:02}, {} {}", 
                date.0, 
                monthify(date.1 as usize).unwrap_or("ERR".to_string()),
                date.2),
            preview: preview
        };
        mapped_posts.insert(0, post);
    }

    return mapped_posts;
}

#[get("/blog")]
pub async fn blog_index() -> Template {
    let num_retrieved = 5;
    let mapped_posts = aggregate_blog_posts(num_retrieved, 0).await;

    // Calculate pagination
    let count = posts::get_post_count().await.unwrap_or(0);
    let pages = count as i64 / num_retrieved;
    let next = pages > 1;
    let pages = if pages > 0 {pages} else {1};

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
pub async fn blog(page: usize) -> Template {
    let num_retrieved = 5;
    let mapped_posts = aggregate_blog_posts(
        num_retrieved, num_retrieved * (page - 1) as i64
    ).await;

    // Calculate pagination
    let count = posts::get_post_count().await.unwrap_or(0);
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
pub async fn blog_post(post_id: String) -> Template {
    let u = Uuid::parse_str(&post_id);
    let post: Option<BlogPost>  = match u {
        Ok(uuid) => posts::retrieve_by_uuid(uuid)
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
