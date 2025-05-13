use actix_web::{web, App, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use dotenvy::dotenv;
use std::{env, io};

mod actions;
mod apis;
mod models;
mod schema;

use crate::apis::book::{all_way, create_book, get_book, get_book_by_inner_join, get_book_by_left_join, get_book_once};
use crate::apis::page::create_page;

// 定义一个用于异步共享的数据库连接池类型
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

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
            .service(create_book)
            .service(get_book)
            .service(get_book_once)
            .service(get_book_by_inner_join)
            .service(get_book_by_left_join)
            .service(create_page)
            .service(all_way)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
