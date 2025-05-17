use ch09_features_relations::{models, pool::establish_connection, schema};
use diesel::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    use models::{Book, Page};
    use schema::books;
    let conn = &mut establish_connection();

    let momo = books::table
        .filter(books::title.eq("Momo"))
        .select(Book::as_select())
        .get_result(conn)?;

    // get pages for a book
    let pages = Page::belonging_to(&momo)
        .select(Page::as_select())
        .load(conn)?;

    println!("Pages for \"Momo\": \n {pages:?}\n");

    Ok(())
}
