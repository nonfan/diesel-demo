use ch09_features_relations::{models, pool::establish_connection, schema};
use diesel::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    use models::{Book, Page};
    use schema::books;
    let conn = &mut establish_connection();

    let all_books = books::table.select(Book::as_select()).load(conn)?;

    // get all pages for all books
    let pages = Page::belonging_to(&all_books)
    .select(Page::as_select())
    .load(conn)?;

    // group the pages per book
    let pages_per_book = pages
    .grouped_by(&all_books)
    .into_iter()
    .zip(all_books)
    .map(|(pages, book)| (book, pages))
    .collect::<Vec<(Book, Vec<Page>)>>();

    println!("Pages per book: \n {pages_per_book:?}\n");

    Ok(())
}
