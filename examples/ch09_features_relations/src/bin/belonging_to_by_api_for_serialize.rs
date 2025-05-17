use ch09_features_relations::{models, pool::establish_connection, schema};
use diesel::prelude::*;
use models::{Book, Page};
use serde::Serialize;

#[derive(Serialize, Debug)] // [!code focus:6]
struct BookWithPages {
    #[serde(flatten)]
    book: Book,
    pages: Vec<Page>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .map(|(pages, book)| BookWithPages { book, pages }) // [!code focus:2]
        .collect::<Vec<BookWithPages>>();

    println!("Pages per book: \n {pages_per_book:?}\n");

    Ok(())
}
