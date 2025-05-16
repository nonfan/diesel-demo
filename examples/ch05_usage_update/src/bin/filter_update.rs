use ch05_usage_update::{establish_connection, schema};
use diesel::prelude::*;
fn main() {
    use schema::posts::dsl::*;

    let conn = &mut establish_connection();

    diesel::update(posts.filter(title.eq("Rust入门")))
        .set(published.eq(true))
        .execute(conn)
        .unwrap();
}
