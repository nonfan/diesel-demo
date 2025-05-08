use actix_web::{App, test, web, HttpRequest};
use diesel::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use rust_demo::DbPool;
use rust_demo::models::{NewUser, User};
use rust_demo::routes::user::{create_user, delete_user, get_user, list_users, update_user};
use serde::Serialize;
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
async fn test_list_users() {
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
