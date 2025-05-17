use ch09_features_relations::{models, pool::establish_connection, schema};
use diesel::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use models::{Author, Book, BookAuthor};
    use schema::{authors, books};

    let conn = &mut establish_connection();

    // 查询作者
    let author_mofan = authors::table
        .filter(authors::name.eq("mofan"))
        .select(Author::as_select())
        .get_result(conn)?;

    // 作者通过中间表 BookAuthor 查询都书，实现多对多
    let books = BookAuthor::belonging_to(&author_mofan)
        .inner_join(books::table)
        .select(Book::as_select())
        .load(conn)?;

    println!("Books by mofan: {books:?}");

    Ok(())
}
