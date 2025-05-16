use ch05_usage_update::{establish_connection, models::UpdatePost, schema};
use diesel::prelude::*;
fn main() {
    use schema::posts::dsl::*;

    let conn = &mut establish_connection();

    let new_post = UpdatePost {
        title: "Rust文章".into(),
        body: "Rust内容".into(),
    };

    diesel::update(posts.filter(id.eq(1)))
        .set(&new_post)
        .execute(conn)
        .unwrap();
}
