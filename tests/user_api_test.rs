use actix_web::{App, test, web};
use diesel::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use rust_demo::DbPool;
use rust_demo::models::{NewUser, User};
use rust_demo::routes::user::{create_user, delete_user, get_user, list_users, update_user};
use serde_json::json;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn setup_test_db() -> DbPool {
    let manager = ConnectionManager::<SqliteConnection>::new(":memory:");
    let pool = Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("无法创建连接池");

    let mut conn = pool.get().expect("无法获取连接");
    conn.run_pending_migrations(MIGRATIONS)
        .expect("无法运行迁移");

    pool
}

#[actix_rt::test]
async fn test_list_users_is_empty() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(list_users),
    )
    .await;

    let req = test::TestRequest::get().uri("/users").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: Vec<User> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 0);
}

#[actix_rt::test]
async fn test_list_users_not_is_empty() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(list_users)
        .service(create_user)
    )
    .await;

    let new_user = NewUser {
        username: "test".into(),
        remark: "nothing".into(),
    };

    let req = test::TestRequest::post()
    .uri("/users")
    .set_json(&new_user)
    .to_request();
    test::call_service(&app, req).await;


    let req = test::TestRequest::get().uri("/users").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let results: Vec<User> = test::read_body_json(resp).await;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].username, "test");
    assert_eq!(results[0].remark, "nothing");
}


#[actix_rt::test]
async fn test_create_user() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_user),
    )
    .await;

    let new_user = NewUser {
        username: "test".into(),
        remark: "nothing".into(),
    };

    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&new_user)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
    let body: User = test::read_body_json(resp).await;
    assert_eq!(body.username, new_user.username);
}

#[actix_rt::test]
async fn test_create_user_error() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(create_user),
    )
    .await;

    let new_user = json!({
        "username": "test",
    });

    let req = test::TestRequest::post()
    .uri("/users")
    .set_json(&new_user)
    .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[actix_rt::test]
async fn test_get_user() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(create_user)
        .service(delete_user)
        .service(get_user),
    )
    .await;

    let new_user = NewUser {
        username: "test".into(),
        remark: "nothing".into(),
    };

    // 创建用户
    let req = test::TestRequest::post()
    .uri("/users")
    .set_json(&new_user)
    .to_request();
    let resp = test::call_service(&app, req).await;
    let created_user: User = test::read_body_json(resp).await;

    let req = test::TestRequest::get()
    .uri(&format!("/users/{}", created_user.id))
    .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let result: User = test::read_body_json(resp).await;

    assert_eq!(result.username, "test");
    assert_eq!(result.remark, "nothing");
}
#[actix_rt::test]
async fn test_get_nonexistent_user() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(create_user)
        .service(delete_user)
        .service(get_user),
    )
    .await;

    let req = test::TestRequest::get().uri("/users/-1").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

#[actix_rt::test]
async fn test_update_user() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(create_user)
            .service(update_user),
    )
    .await;

    let new_user = NewUser {
        username: "test".into(),
        remark: "nothing".into(),
    };

    // 创建用户
    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&new_user)
        .to_request();
    let resp = test::call_service(&app, req).await;
    let body: User = test::read_body_json(resp).await;

    // 更新用户
    let updated_user = json!({
        "username": "update test",
        "remark": "更新备注"
    });
    let req = test::TestRequest::put()
        .uri(&format!("/users/{}", body.id))
        .set_json(&updated_user)
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);

    let body: User = test::read_body_json(resp).await;
    assert_eq!(body.username, "update test");
    assert_eq!(body.remark, "更新备注");
}

#[actix_rt::test]
async fn test_update_user_error() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(create_user)
        .service(update_user),
    )
    .await;

    let new_user = NewUser {
        username: "test".into(),
        remark: "nothing".into(),
    };

    // 创建用户
    let req = test::TestRequest::post()
    .uri("/users")
    .set_json(&new_user)
    .to_request();
    let resp = test::call_service(&app, req).await;
    let body: User = test::read_body_json(resp).await;

    // 更新用户
    let updated_user = json!({
        "username": "update test",
    });
    let req = test::TestRequest::put()
    .uri(&format!("/users/{}", body.id))
    .set_json(&updated_user)
    .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

#[actix_rt::test]
async fn test_delete_user() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(create_user)
        .service(delete_user),
    )
    .await;

    let new_user = NewUser {
        username: "test".into(),
        remark: "nothing".into(),
    };

    // 创建用户
    let req = test::TestRequest::post()
    .uri("/users")
    .set_json(&new_user)
    .to_request();
    let resp = test::call_service(&app, req).await;
    let created_user: User = test::read_body_json(resp).await;

    // 删除用户
    let req = test::TestRequest::delete()
    .uri(&format!("/users/{}", created_user.id))
    .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    // 验证删除
    let req = test::TestRequest::get()
    .uri(&format!("/users/{}", created_user.id))
    .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

// 测试无效的JSON输入
#[actix_rt::test]
async fn test_create_user_invalid_json() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(create_user),
    )
    .await;

    let invalid_json = "{"; // 无效的JSON字符串
    let req = test::TestRequest::post()
    .uri("/users")
    .set_payload(invalid_json)
    .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400);
}

// 测试更新不存在的用户
#[actix_rt::test]
async fn test_update_nonexistent_user() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(update_user),
    )
    .await;

    let updated_user = json!({
        "username": "update test",
        "remark": "更新备注"
    });
    let req = test::TestRequest::put()
    .uri("/users/-1")
    .set_json(&updated_user)
    .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

// 测试删除不存在的用户
#[actix_rt::test]
async fn test_delete_nonexistent_user() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(delete_user),
    )
    .await;

    let req = test::TestRequest::delete()
    .uri("/users/9999999")
    .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404);
}

// 测试无效ID格式
#[actix_rt::test]
async fn test_get_user_invalid_id() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(get_user),
    )
    .await;

    let req = test::TestRequest::get()
    .uri("/users/invalid")
    .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 404); // 假设路由会返回400来处理无效ID
}

// 测试空字符串输入
#[actix_rt::test]
async fn test_create_user_empty_strings() {
    let pool = setup_test_db();
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(create_user),
    )
    .await;

    let new_user = json!({
        "username": "",
        "remark": ""
    });

    let req = test::TestRequest::post()
    .uri("/users")
    .set_json(&new_user)
    .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 400); // 假设有验证逻辑拒绝空字符串
}

// 测试数据库连接错误
#[actix_rt::test]
async fn test_database_connection_error() {
    // 创建一个无效的数据库连接池
    let manager = ConnectionManager::<SqliteConnection>::new("invalid_database");
    let pool = Pool::builder()
    .max_size(1)
    .build(manager)
    .expect("无法创建连接池");

    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(list_users),
    )
    .await;

    let req = test::TestRequest::get().uri("/users").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 500); // 假设数据库错误返回500
}
#[actix_rt::test]
async fn test_update_user_db_error() {
    // 创建一个无效的数据库连接池（模拟数据库连接失败）
    let manager = ConnectionManager::<SqliteConnection>::new("invalid_database_path");
    let pool = Pool::builder()
    .max_size(1)
    .build(manager)
    .expect("无法创建连接池");

    // 初始化 Actix Web 应用
    let app = test::init_service(
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(update_user)
        .service(delete_user)
        .service(get_user),
    )
    .await;

    // 构造请求数据
    let updated_user = json!({
        "username": "update_test",
        "remark": "更新备注"
    });
    let req = test::TestRequest::put()
    .uri("/users/1")
    .set_json(&updated_user)
    .to_request();

    // 发送请求
    let resp = test::call_service(&app, req).await;

    // 验证响应状态码为 500（Internal Server Error）
    assert_eq!(resp.status(), 500);

    let req = test::TestRequest::get()
    .uri("/users/1")
    .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 500);

    let req = test::TestRequest::delete()
    .uri("/users/1")
    .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 500);
}