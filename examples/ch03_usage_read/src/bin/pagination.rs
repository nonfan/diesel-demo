use ch03_usage_read::{establish_connection, models, schema};
fn main() {
    use diesel::prelude::*;
    use models::Post;
    use schema::posts::dsl::posts;

    let connection = &mut establish_connection();

    let results = posts // [!code focus:5]
        .limit(3)
        .offset(3) // 获取第2页的数据
        .load::<Post>(connection)
        .expect("Error loading posts");

    for post in results {
        println!("标题：{}", post.title);
        println!("内容：{}", post.body);
        println!("------------------");
    }
}
