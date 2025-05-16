use ch05_usage_update::{establish_connection, schema};
use diesel::prelude::*;
fn main() {
    use schema::posts::dsl::*;

    let conn = &mut establish_connection();

    diesel::update(posts)
        .set(published.eq(true))
        .execute(conn)
        .unwrap();
}
