use dnguyen_blog::posts;

mod common;

#[tokio::test]
async fn it_gets_recents() {
    common::db::reset("blog_posts").await.expect("Error resetting table: blog_posts");
    // Panic if we can't generate test data
    let mut uuids = common::db::create_random_posts(20).await.unwrap();

    // Test will fail if result is not Ok(_)
    let posts = posts::retrieve_recent().await.unwrap();

    assert_eq!(posts.len(), 10);

    // Posts should appear in reverse chronological order.
    // Since new UUIDs are pushed into a vector, the last
    // element of `uuids` should be equal to the first
    // element of posts.
    
    for post in posts.iter() {
        assert_eq!(post.uuid, uuids.pop().unwrap());
    }
}
