use dnguyen_blog::posts;

mod common;

#[tokio::test]
async fn it_gets_recents() {
    common::db::reset("blog_posts").await.expect("Error resetting table: blog_posts");
    // Panic if we can't generate test data
    let mut uuids = common::db::create_random_posts(20).await.unwrap();

    // Test will fail if result is not Ok(_)
    let posts = posts::retrieve_recent(10).await.unwrap();

    assert_eq!(posts.len(), 10);

    // Posts should appear in reverse chronological order.
    // Since new UUIDs are pushed into a vector, the last
    // element of `uuids` should be equal to the first
    // element of posts.
    
    for post in posts.iter() {
        assert_eq!(post.uuid, uuids.pop().unwrap());
    }
}

#[tokio::test]
async fn it_casts_row_to_blog_post() {

    use std::convert::TryFrom;
    use chrono::prelude::*;
    use uuid::Uuid;

    common::db::reset("blog_posts").await.expect("Error resetting table: blog_posts");
    common::db::create_random_posts(10).await.unwrap();
    
    let row = common::db::get_first_post().await.unwrap();
    let post = posts::BlogPost::try_from(&row).unwrap();

    assert_eq!(post.uuid, row.get::<&str, Uuid>("id"));
    assert_eq!(post.created_at, row.get::<&str, DateTime<Utc>>("created_at"));
    assert_eq!(post.updated_at, row.get::<&str, Option<DateTime<Utc>>>("updated_at"));
    assert_eq!(post.published_at, row.get::<&str, Option<DateTime<Utc>>>("published_at"));
    assert_eq!(post.is_public, row.get::<&str, bool>("is_public"));
    assert_eq!(post.delta, row.get::<&str, Option<serde_json::Value>>("delta"));
    assert_eq!(post.title, row.get::<&str, String>("title"));
}
