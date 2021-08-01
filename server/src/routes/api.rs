
pub mod blog_posts {
    use rocket::{get, post};
    use rocket::response::status;
    use rocket::serde::json::Json;

    use dnguyen_blog::model::posts;
    use dnguyen_blog::model::users::User;
    use dnguyen_blog::http::dto::CreatePostArgs;

    /// By default, retrieve 10 most recent posts
    #[get("/posts")]
    pub async fn recent() -> Option<String> {
        let post = match posts::retrieve_recent(10).await {
            Ok(r) => match serde_json::to_string(&r) {
                Ok(v) => Some(v),
                Err(_) => None,
            },
            Err(_) => None,
        };

        return post;
    }

    /// Retreives a number of recent posts in JSON format
    #[get("/posts?<count>")]
    pub async fn recent_count(count: i64) -> Option<String> {
        let post = match posts::retrieve_recent(count).await {
            Ok(r) => match serde_json::to_string(&r) {
                Ok(v) => Some(v),
                Err(_) => None,
            },
            Err(_) => None,
        };

        return post;
    }

    /// Create a new post with arguments from posted JSON
    #[post("/posts/draft", format = "json", data = "<args>")]
    pub async fn new(user: User, args: Json<CreatePostArgs>) -> status::Accepted<()> {
        println!("user {} posted a draft.", user.id);
        posts::create(args.into_inner()).await.unwrap();
        return status::Accepted(Some(()));
    }
}
