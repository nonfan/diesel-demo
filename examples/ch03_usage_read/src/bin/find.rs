use ch03_usage_read::{establish_connection, models, schema};
fn main() {
    use diesel::prelude::*;
    use models::Post;
    use schema::posts::dsl::posts;

    let connection = &mut establish_connection();

    let post = posts // [!code focus:4]
        .find(1)
        .first::<Post>(connection)
        .expect("Error loading posts");

    println!("标题：{}", post.title);
    println!("内容：{}", post.body);
    println!("------------------");
}
