use ch06_usage_delete::{establish_connection, models::Post, schema};
use diesel::prelude::*;

fn main() {
    use schema::posts::dsl::*;

    let conn = &mut establish_connection();

    let post = posts.filter(id.eq(1)).first::<Post>(conn)?;

    diesel::delete(&post)
        .execute(conn)
        .expect("Error deleting posts");
}
