use ch07_features_transaction::{establish_connection, schema, models::Post};

fn main() {
    use schema::posts::dsl::*;
    use diesel::prelude::*;

    let connection = &mut establish_connection();

    let result = connection.transaction::<_, diesel::result::Error, _>(|conn| {
        // 在事务中执行数据库操作
        let new_post = diesel::insert_into(posts)
            .values((title.eq("Rust"), body.eq("Rust 内容")))
            .get_result::<Post>(conn)?;

        diesel::update(&new_post)
            .set(published.eq(true))
            .execute(conn)?;

        Ok(())
    });

    match result {
        Ok(_) => println!("Success"),
        Err(error) => println!("Error: {}", error),
    }
}
