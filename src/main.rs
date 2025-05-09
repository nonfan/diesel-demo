
use actix_web::{App, HttpResponse, HttpServer, Responder, Result, error, get, web};
use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl, r2d2};
use dotenvy::dotenv;
use std::{env, io};
use diesel::prelude::*;

use rust_demo::models::{Page,Book};
use rust_demo::schema::{books,pages};

// 定义一个用于异步共享的数据库连接池类型
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[get("/")]
async fn get_book(pool: web::Data<DbPool>) -> Result<impl Responder> {
    // use crate::schema::books::dsl::*;

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    // let result = web::block(move || books.filter(title.eq("Momo")).first::<Book>(&mut conn))
    //     .await?
    //     .map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || {

        let momo = books::table
        .filter(books::title.eq("Momo"))
        .select(Book::as_select())
        .get_result(&mut conn)?;

        // get pages for a book
        let pages = Page::belonging_to(&momo)
        .select(Page::as_select())
        .load(&mut conn);


    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // 获取数据库文件地址
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // 初始化数据库连接池
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("Failed to build pool: {}", e))
        })?;

    HttpServer::new(move || {
        App::new()
            // 克隆连接池，以共享连接池
            .app_data(web::Data::new(pool.clone()))
            .service(get_book)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
