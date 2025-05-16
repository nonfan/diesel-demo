use ch05_usage_update::{establish_connection, schema};
use diesel::prelude::*;
fn main() {
    use schema::posts::dsl::*;

    let conn = &mut establish_connection();

    diesel::update(posts.filter(title.eq("Rust入门")))
        .set((
            title.eq("Rust快速开始"),
            body.eq("可以直接将多个字段更新组合为一个元组传入"),
        ))
        .execute(conn)
        .unwrap();
}
