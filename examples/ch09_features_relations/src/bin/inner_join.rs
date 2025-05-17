use ch09_features_relations::{models, pool::establish_connection, schema};
use diesel::prelude::*;
use models::{Page, Book};
fn main() -> Result<(), diesel::result::Error> {

    use schema::{books, pages};

    let conn = &mut establish_connection();

    let page_with_book = pages::table     // [!code focus:7]
    .inner_join(books::table)
    .filter(books::title.eq("Momo"))
    .select((Page::as_select(), Book::as_select()))
    .load::<(Page, Book)>(conn)?;

    println!("Page-Book pairs: {page_with_book:?}");

    Ok(())
}