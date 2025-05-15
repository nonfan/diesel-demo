use diesel::prelude::*;
use std::env::args;
use ch01_blog_demo_cli::{establish_connection, schema};

fn main() {
    use schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
    .execute(connection)
    .expect("删除帖子时出错");

    println!("删除 {} 篇帖子", num_deleted);
}