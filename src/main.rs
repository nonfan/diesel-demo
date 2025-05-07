use crate::models::{NewUser, User};
use actix_web::{
    App, HttpResponse, HttpServer, Responder, Result, delete, error, get, post, put, web,
};
use diesel::r2d2::{self,ConnectionManager};
use serde_json::json;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, io};
use crate::schema::users::dsl::*;

pub mod models;
pub mod schema;
type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

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
    let data = new_user.clone();

    let mut conn = pool.get().map_err(|e| error::ErrorInternalServerError(e))?;

    web::block(move || {
        diesel::insert_into(users)
            .values(&new_user)
        .execute(&mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({"message": "创建用户成功","data": data})))
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

    web::block(move || {
        diesel::update(users)
        .filter(id.eq(user_id))
        .set((username.eq(new_user.username), remark.eq(new_user.remark)))
        .execute(&mut conn)
    })
    .await?
    .map_err(|e| error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({"message": "修改成功"})))
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
        .execute(&mut conn)
    })
    .await?;

    match deleted_user {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                return Ok(HttpResponse::NotFound().json(json!({ "message": "用户不存在" })))
            }

            Ok(HttpResponse::Ok().json(json!({ "message": "删除成功" })))
        },
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

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
