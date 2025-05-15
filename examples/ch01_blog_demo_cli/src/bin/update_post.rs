use ch01_blog_demo_cli::establish_connection;
use ch01_blog_demo_cli::models::Post;
use ch01_blog_demo_cli::schema;
use diesel::prelude::*;
use std::env::args;

fn main() {
    use schema::posts::dsl::{posts,published};

    // 通过命令行获取文章ID
    let post_id: i32 = match args().nth(1) {
        Some(post_id) => post_id.parse::<i32>().unwrap(),
        None => {
            panic!("命令行获取ID失败")
        }
    };

    println!("Updating post {:?}", post_id);

    let conn = &mut establish_connection();

    let post = conn
    .transaction(|conn| {

        let post = posts.find(post_id).select(Post::as_select()).first(conn)?;

        diesel::update(posts.find(post_id))
        .set(published.eq(true))
        .execute(conn)?;

        Ok(post)
    })
    .unwrap_or_else(|_: diesel::result::Error| panic!("Unable to find post {}", post_id));

    println!("Published post {}", post.title);
}