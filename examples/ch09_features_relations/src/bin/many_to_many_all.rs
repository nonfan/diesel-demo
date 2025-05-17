use ch09_features_relations::{models, pool::establish_connection, schema};
use diesel::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use models::{Author, Book, BookAuthor};
    use schema::{authors, books};

    let conn = &mut establish_connection();

    // 先查询所有作者，返回 Vec<Author>
    let all_authors: Vec<Author> = authors::table.select(Author::as_select()).load(conn)?;

    // 根据所有作者查询关联的 BookAuthor 并关联 books 表，查询结果为 Vec<(BookAuthor, Book)>
    let books_with_join: Vec<(BookAuthor, Book)> = BookAuthor::belonging_to(&all_authors)
    .inner_join(books::table)
    .select((BookAuthor::as_select(), Book::as_select()))
    .load(conn)?;

    // 使用 grouped_by 进行分组，grouped_by 接收的参数是所有者列表 (authors)
    // 返回类型是 Vec<Vec<(BookAuthor, Book)>>，每个内层 Vec 是对应作者的多本书和关联信息
    let grouped_books = books_with_join.grouped_by(&all_authors);

    // 把 grouped_books 和 all_authors 组合，转成 (Author, Vec<Book>) 形式
    let books_per_author: Vec<(Author, Vec<Book>)> = grouped_books
    .into_iter()
    .zip(all_authors.into_iter())
    .map(|(book_authors, author)| {
        // book_authors 是 Vec<(BookAuthor, Book)>，取出第二个元素 Book 收集成 Vec<Book>
        let books = book_authors.into_iter().map(|(_, book)| book).collect();
        (author, books)
    })
    .collect();

    println!("All authors including their books: {books_per_author:?}");

    Ok(())
}
