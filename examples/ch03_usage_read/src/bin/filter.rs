use ch03_usage_read::{establish_connection, models, schema};
fn main() {
    use diesel::prelude::*;
    use models::Post;
    use schema::posts::dsl::{posts, published};

    let connection = &mut establish_connection();

    let results = posts // [!code focus:4]
        .filter(published.eq(true))
        .load::<Post>(connection)
        .expect("Error loading posts");

    for post in results {
        println!("标题：{}", post.title);
        println!("内容：{}", post.body);
        println!("------------------");
    }
}
