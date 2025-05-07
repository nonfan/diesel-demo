use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, Result, delete, error, get, post, put, web,
};
use serde_json::json;
use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use std::{env, io};
pub mod models;
pub mod schema;
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/users")]
async fn list_users(pool: web::Data<DbPool>) -> Result<impl Responder> {
    use crate::schema::users::dsl::*;

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let results = web::block(move || users.load::<User>(&mut conn))
        .await?
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(results))
}

#[get("/users/{id}")]
async fn get_user(pool: web::Data<DbPool>, path: web::Path<i32>) -> Result<impl Responder> {
    let user_id = path.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || users.filter(id.eq(user_id)).first::<User>(&mut conn))
        .await?
        .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}

#[post("/users")]
async fn create_posts(pool: web::Data<DbPool>, body: web::Json<NewUser>) -> Result<impl Responder> {
    use crate::schema::users::dsl::*;

    let new_user = body.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || {
        diesel::insert_into(users)
            .values(new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}

#[put("/users/{id}")]
async fn update_user(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
    body: web::Json<NewUser>,
) -> Result<impl Responder> {
    let user_id = path.into_inner();
    let new_user = body.into_inner();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    let result = web::block(move || {
        diesel::update(users)
            .filter(id.eq(user_id))
            .set((username.eq(new_user.username), remark.eq(new_user.remark)))
            .execute(&mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/users/{id}")]
async fn delete_user(
    pool: web::Data<DbPool>,
    path: web::Path<i32>,
) -> Result<impl Responder> {
    let user_id = path.into_inner();
    let mut conn = pool.get().map_err(error::ErrorInternalServerError)?;

    let deleted_user = web::block(move || {
        diesel::delete(users.filter(id.eq(user_id)))
        .returning(User::as_returning())
        .get_result(&mut conn) // 返回被删除的行
    })
    .await?;

    match deleted_user {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(diesel::NotFound) => Ok(HttpResponse::NotFound().json(json!({ "error": "用户不存在" }))),
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(list_users)
            .service(get_user)
            .service(create_posts)
            .service(update_user)
            .service(delete_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
