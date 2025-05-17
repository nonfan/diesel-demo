use ch07_features_transaction::{establish_connection, models::Post, schema};

fn main() {
    use diesel::prelude::*;
    use schema::posts::dsl::*;

    let connection = &mut establish_connection();

    let result = connection.transaction::<_, diesel::result::Error, _>(|conn| {
        // 在事务中执行数据库操作
        let new_post = diesel::insert_into(posts)
            .values((title.eq("Rust"), body.eq("Rust 内容")))
            .get_result::<Post>(conn)?;

        // 嵌套事务（保存点）
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            // 内层事务：尝试更新post
            diesel::update(&new_post)
                .set(published.eq(true))
                .execute(conn)?;

            Ok(())
        })?;

        Ok(())
    });

    match result {
        Ok(_) => println!("Success"),
        Err(error) => println!("Error: {}", error),
    }
}
