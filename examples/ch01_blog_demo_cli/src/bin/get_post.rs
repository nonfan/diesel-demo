use ch01_blog_demo_cli::establish_connection;
use ch01_blog_demo_cli::models::Post;
use ch01_blog_demo_cli::schema;
use diesel::prelude::*;
use std::env::args;

fn main() {
    use schema::posts::dsl::posts;

    // 通过命令行获取文章ID
    let post_id: i32 = match args().nth(1) {
        Some(post_id) => post_id.parse::<i32>().unwrap(),
        None => {
            panic!("命令行获取ID失败")
        }
    };

    let connection = &mut establish_connection();

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional();

    match post {
        Ok(Some(post)) => println!("文章ID: {} \n标题: {}", post.id, post.title),
        Ok(None) => println!("找不到文章 {}", post_id),
        Err(_) => println!("获取帖子时发生错误 {}", post_id),
    }
}
