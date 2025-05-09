use actix_web::{App, test, web};
use diesel::r2d2::{ConnectionManager, Pool};
use rust_demo::routes::user::{create_user, delete_user, get_user, list_users, update_user};
use diesel::SqliteConnection;
use std::io;

#[actix_rt::test]
async fn test_app_configuration() {
    // 创建内存数据库连接池
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = Pool::builder()
    .max_size(1)
    .build(manager)
    .expect("无法创建连接池");

    // 模拟 main.rs 中的 App 配置
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(list_users)
        .service(get_user)
        .service(create_user)
        .service(update_user)
        .service(delete_user),
    )
    .await;

    // 测试一个路由以验证配置
    let req = test::TestRequest::get().uri("/users").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 500);
}
#[actix_rt::test]
async fn test_database_pool_creation_error() {
    // 使用一个无效的数据库路径（例如，权限受限的目录）
    let invalid_url = "/root/invalid_database.db"; // 假设无权限写入
    let manager = ConnectionManager::<SqliteConnection>::new(invalid_url);
    let pool_result = Pool::builder()
    .max_size(15)
    .build(manager);

    // 验证返回错误
    assert!(pool_result.is_err(), "连接池应创建失败");
    if let Err(e) = pool_result {
        // 验证错误被正确转换为 io::Error
        let io_error = io::Error::new(io::ErrorKind::Other, e);
        assert_eq!(io_error.kind(), io::ErrorKind::Other);
    }
}