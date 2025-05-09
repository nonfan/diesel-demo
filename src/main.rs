use actix_web::{
    App, HttpServer, web,
};
use rust_demo::actions::utils::establish_connection;
use rust_demo::routes::user::{create_user, delete_user, get_user, list_users, update_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pool = establish_connection()?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(list_users)
            .service(get_user)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind("127.0.0.1:9080")?
    .run()
    .await
}
