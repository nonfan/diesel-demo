use ch09_features_relations::{models, pool::establish_connection, schema};
use diesel::prelude::*;
use models::{Book, Page};
fn main() -> Result<(), diesel::result::Error> {
    use schema::{books, pages};

    let conn = &mut establish_connection();

    let book_without_pages = books::table // [!code focus:6]
        .left_join(pages::table)
        .select((Book::as_select(), Option::<Page>::as_select()))
        .load::<(Book, Option<Page>)>(conn)?;

    println!("Book-Page pairs (including empty books): {book_without_pages:?}");

    Ok(())
}
