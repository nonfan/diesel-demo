use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use std::io;
use dotenvy::dotenv;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> io::Result<DbPool> {
    dotenv().ok(); // 加载 .env 文件，忽略加载失败
    let database_url = env::var("DATABASE_URL")
    .map_err(|e| io::Error::new(io::ErrorKind::NotFound, format!("DATABASE_URL missing: {}", e)))?;

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
    .max_size(10)
    .build(manager)
    .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to build pool: {}", e)))?;

    Ok(pool)
}