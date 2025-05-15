use ch03_usage_read::{establish_connection, models, schema};
use ch03_usage_read::schema::posts::title;

fn main() {
    use diesel::prelude::*;
    use models::Post;
    use schema::posts::dsl::{posts};

    let connection = &mut establish_connection();

    let results = posts // [!code focus:4]
        .order(title.asc())
        .load::<Post>(connection)
        .expect("Error loading posts");

    for post in results {
        println!("标题：{}", post.title);
        println!("内容：{}", post.body);
        println!("------------------");
    }
}
