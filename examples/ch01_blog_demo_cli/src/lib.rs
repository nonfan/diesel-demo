use diesel::prelude::*;
use std::env;
use dotenvy::dotenv;
pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    // 让我们可以获取环境变量 .env 内容
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}