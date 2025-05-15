use ch01_blog_demo_cli::establish_connection;
use ch01_blog_demo_cli::schema;
use ch01_blog_demo_cli::models::Post;
use diesel::prelude::*;

fn main() {
    use schema::posts::dsl::*;

    let conn  = &mut establish_connection();

    let results = posts
    .filter(published.eq(true))
    .limit(5)
    .select(Post::as_select())
    .load::<Post>(conn)
    .expect("加载Posts数据发生异常");

    println!("展示 {} 篇文章", results.len());
    println!("************");
    for post in results {
        println!("Title: {}", post.title);
        println!("Body {}", post.body);
        println!("-----------\n");
    }
}