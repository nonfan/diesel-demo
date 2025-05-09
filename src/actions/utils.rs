use std::{env};
use std::io::{self, Error, ErrorKind};
use diesel::r2d2::{self, ConnectionManager}; // 全部用 diesel::r2d2
use diesel::SqliteConnection;
use dotenvy::dotenv;
use crate::DbPool;

pub fn establish_connection() -> Result<DbPool, Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
    .map_err(|_| Error::new(ErrorKind::NotFound, "Environment variable DATABASE_URL is not set"))?;

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    r2d2::Pool::builder()
    .max_size(15)
    .build(manager)
    .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))
}
//
// #[cfg(test)]
// mod tests {
//     // use super::*;
//     use std::fs;
//     use super::establish_connection;
//
//     #[test]
//     fn test_establish_connection_success() {
//         // 准备：设置测试数据库路径
//         let test_db_path = "test.db";
//         unsafe {
//             std::env::set_var("DATABASE_URL", test_db_path);
//         }
//
//         // 创建一个空的 SQLite 文件（如果不存在）
//         let _ = fs::File::create(test_db_path);
//
//         // 执行连接
//         let result = establish_connection();
//
//         // 断言成功
//         assert!(result.is_ok(), "Expected Ok(DbPool), got {:?}", result);
//
//         // 清理：移除测试数据库文件
//         let _ = fs::remove_file(test_db_path);
//     }
//
//     #[test]
//     fn test_establish_connection_fail_when_env_missing() {
//         // 清除 DATABASE_URL 环境变量
//         {
//             std::env::remove_var("DATABASE_URL");
//         }
//
//         let result = establish_connection();
//
//         assert!(result.is_err(), "Expected error due to missing DATABASE_URL");
//     }
//
// }
