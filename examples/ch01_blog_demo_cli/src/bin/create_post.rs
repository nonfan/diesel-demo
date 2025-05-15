use ch01_blog_demo_cli::establish_connection;
use ch01_blog_demo_cli::models::{NewPost, Post};
use ch01_blog_demo_cli::schema::posts;
use diesel::prelude::*;
fn main() {
    let conn = &mut establish_connection();

    let new_post = NewPost {
        title: "Rust 快速开始".into(),
        body: "关于Rust如何快速开始".into(),
    };

    conn.transaction(|conn| {
        diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)?;

        posts::table
        .order(posts::id.desc())
        .select(Post::as_select())
        .first(conn)
    })
    .expect("Error while saving post");
}
