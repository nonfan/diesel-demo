use crate::models::{NewPage, Page};
use crate::schema::books;
use crate::{DbPool};
use actix_web::{error, post, web, HttpResponse, Responder, Result};
use diesel::prelude::*;

#[post("/books/{book_id}/pages")]
async fn create_page(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body: web::Json<NewPage>,
) -> Result<impl Responder> {
    let path_book_id = path.into_inner();
    let mut new_page = body.into_inner();
    new_page.book_id = Some(path_book_id);

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || -> Result<_, diesel::result::Error> {
        use crate::schema::pages::dsl::*;
        //检查书籍是否存在
        let book_exists = books::table
            .filter(books::id.eq(path_book_id))
            .select(books::id)
            .first::<i32>(&mut conn)
            .optional()?
            .is_some();
        if !book_exists {
            return Err(diesel::result::Error::NotFound);
        }

        let page_result = diesel::insert_into(pages)
        .values(new_page)
        .returning(Page::as_returning())
        .get_result(&mut conn)?;

        Ok(page_result)
    })
    .await?
    .map_err(|e| match e {
        diesel::result::Error::NotFound => error::ErrorNotFound("The book not found"),
        _ => error::ErrorInternalServerError(e),
    })?;

    Ok(HttpResponse::Created().json(result))
}
