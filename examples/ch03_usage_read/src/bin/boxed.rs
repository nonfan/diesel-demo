use ch03_usage_read::{establish_connection, models, schema};
fn main() {
    use diesel::prelude::*;
    use models::Post;
    use schema::posts::dsl::{posts, published}; // [!code focus]

    let connection = &mut establish_connection();

    let query = posts.filter(published.eq(true)).limit(6).into_boxed(); // [!code focus:5]

    let results = query
        .load::<Post>(connection)
        .expect("Error loading posts");

    for post in results {
        println!("标题：{}", post.title);
        println!("内容：{}", post.body);
        println!("------------------");
    }
}
