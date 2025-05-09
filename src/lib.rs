use diesel::{r2d2, SqliteConnection};
use diesel::r2d2::ConnectionManager;

pub mod routes;  // 声明 routes 模块（公开）
pub mod models;
pub mod schema;
pub mod actions;
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
