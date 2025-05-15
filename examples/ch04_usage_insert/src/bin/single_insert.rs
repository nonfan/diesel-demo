use ch04_usage_insert::establish_connection;
use ch04_usage_insert::models::{NewPost, Post};
use ch04_usage_insert::schema::posts::dsl::posts;
use diesel::prelude::*;

fn main() {
    let conn = &mut establish_connection();

    let new_post = NewPost {
        title: "Rust 插入".into(),
        body: "插入数据是将 Rust 结构体的数据保存到数据库表的过程".into(),
    };

    let post = diesel::insert_into(posts)
        .values(&new_post)
        .get_result::<Post>(conn)
        .unwrap();

    println!("POST ID：{}", post.id);
    println!("标题：{}", post.title);
    println!("内容：{}", post.body);
}
