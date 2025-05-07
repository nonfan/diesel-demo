use actix_web::{
    App, HttpServer, Responder, web,
};
use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use dotenvy::dotenv;
use std::{env, io};
pub mod models;
pub mod schema;
pub mod handlers;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init();
    dotenv().ok();
    use handlers::{get_user,list_users,create_posts,update_user,delete_user};

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
    .bind("127.0.0.1:9080")?
    .run()
    .await
}
