use crate::models::{Book, NewBook, Page, BookAuthor, Author};
use crate::schema::{books, pages, authors,books_authors};
use crate::DbPool;
use actix_web::{error, get, post, web, HttpResponse, Responder, Result};
use diesel::prelude::*;
use serde::Serialize;

#[post("/books")]
async fn create_book(pool: web::Data<DbPool>, body: web::Json<NewBook>) -> Result<impl Responder> {
    use crate::schema::books::dsl::*;

    let new_book = body.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || {
        diesel::insert_into(books)
            .values(new_book)
            .returning(Book::as_returning())
            .get_result(&mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}

#[derive(Serialize)]
struct BookWithPages {
    #[serde(flatten)]
    book: Book,
    pages: Vec<Page>,
}
#[get("/books/{id}")]
async fn get_book(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || -> Result<_, diesel::result::Error> {
        let book = books::table
            .filter(books::id.eq(id))
            .select(Book::as_select())
            .get_result(&mut conn)?;

        // 查询相关页面
        let book_pages = Page::belonging_to(&book)
            .select(Page::as_select())
            .load(&mut conn)?;

        Ok(BookWithPages {
            book,
            pages: book_pages,
        })
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}


#[derive(Serialize)]
struct BookWithPagesOnce {
    #[serde(flatten)]
    book: Option<Book>,
    pages: Vec<Page>,
}

#[get("/books-once/{id}")]
async fn get_book_once(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || -> Result<_, diesel::result::Error> {
        let book_with_pages = books::table
        .left_join(pages::table.on(books::id.eq(pages::book_id)))
        .filter(books::id.eq(id))
        .select((Book::as_select(), Option::<Page>::as_select()))
        .load::<(Book, Option<Page>)>(&mut conn)?
        .into_iter()
        .fold(BookWithPagesOnce {
            book: None,
            pages: Vec::new(),
        }, |mut acc, (book, page)| {
            if acc.book.is_none() {
                acc.book = Some(book);
            }
            if let Some(page) = page {
                acc.pages.push(page);
            }
            acc
        });

        Ok(book_with_pages)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/books-inner-join/{id}")]
async fn get_book_by_inner_join(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let id = path.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || {
        pages::table
            .inner_join(books::table)
            .filter(books::id.eq(id))
            .select((Page::as_select(), Book::as_select()))
            .load::<(Page, Book)>(&mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/books-left-join/{id}")]
async fn get_book_by_left_join(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let id = path.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || {
        books::table
            .left_join(pages::table)
            .select((Book::as_select(), Option::<Page>::as_select()))
            .load::<(Book, Option<Page>)>(&mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/all-way/{id}")]
async fn all_way(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder> {
    let id = path.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    // let result = web::block(move || {
    //     let astrid_linden = authors::table
    //     .filter(authors::name.eq("李明"))
    //     .select(Author::as_select())
    //     .get_result(&mut conn)?;
    //
    //     // get all of Astrid Linden's books
    //     BookAuthor::belonging_to(&astrid_linden)
    //     .inner_join(books::table)
    //     .select(Book::as_select())
    //     .load(&mut conn)
    // })
    // let result = web::block(move || -> Result<_, diesel::result::Error> {
    //     let books_with_pages = authors::table
    //     .filter(authors::name.eq("李明"))
    //     .inner_join(books_authors::table.on(authors::id.eq(books_authors::author_id)))
    //     .inner_join(books::table.on(books_authors::book_id.eq(books::id)))
    //     .left_join(pages::table.on(books::id.eq(pages::book_id)))
    //     .select((Book::as_select(), Option::<Page>::as_select()))
    //     .load::<(Book, Option<Page>)>(&mut conn)?
    //     .into_iter()
    //     .fold(Vec::<BookWithPages>::new(), |mut acc, (book, page)| {
    //         // 查找是否已有该书籍
    //         if let Some(book_with_pages) = acc.iter_mut().find(|bwp| bwp.book.id == book.id) {
    //             // 如果书籍已存在，添加页面
    //             if let Some(page) = page {
    //                 book_with_pages.pages.push(page);
    //             }
    //         } else {
    //             // 新书籍，创建 BookWithPages
    //             let mut pages = Vec::new();
    //             if let Some(page) = page {
    //                 pages.push(page);
    //             }
    //             acc.push(BookWithPages {
    //                 book,
    //                 pages,
    //             });
    //         }
    //         acc
    //     });
    //
    //     Ok(books_with_pages)
    // })
    let result = web::block(move ||{
        let all_books = books::table.select(Book::as_select()).load(&mut conn)?;

        // get all pages for all books
        let all_pages = Page::belonging_to(&all_books)
        .select(Page::as_select())
        .load(&mut conn)?;

        let pages_per_book = all_pages
        .grouped_by(&all_books)
        .into_iter()
        .zip(all_books)
        .map(|(pages, book)| (book, pages))
        .collect::<Vec<(Book, Vec<Page>)>>();

        Ok::<_, diesel::result::Error>(pages_per_book)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;


    Ok(HttpResponse::Ok().json(result))
}