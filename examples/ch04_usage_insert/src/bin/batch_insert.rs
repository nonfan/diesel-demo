use ch04_usage_insert::establish_connection;
use ch04_usage_insert::models::{NewPost, Post};
use ch04_usage_insert::schema::posts::dsl::posts;
use diesel::prelude::*;

fn main() {
    let conn = &mut establish_connection();

    let new_posts = vec![
        NewPost {
            title: "帖子 1".into(),
            body: "内容 1".into(),
        },
        NewPost {
            title: "帖子 2".into(),
            body: "内容 2".into(),
        },
    ];

    let results = diesel::insert_into(posts)
        .values(&new_posts)
        .get_results::<Post>(conn) // [!code highlight]
        .unwrap();

    for post in results {
        println!("POST ID：{}", post.id);
        println!("标题：{}", post.title);
        println!("内容：{}", post.body);
    }
}
