use ch06_usage_delete::{establish_connection, schema};
use diesel::prelude::*;

fn main() {
    use schema::posts::dsl::*;

    let conn = &mut establish_connection();

    diesel::delete(posts.filter(id.eq(1)))
        .execute(conn)
        .expect("Error deleting posts");
}
