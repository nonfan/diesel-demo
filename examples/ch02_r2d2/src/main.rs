use actix_web::{web, App, HttpServer};
mod pool;
use pool::create_db_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = create_db_pool()?;

    HttpServer::new(move || {
        App::new()
        // 克隆连接池，以共享连接池
        .app_data(web::Data::new(pool.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}