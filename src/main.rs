use actix_web::{middleware::Logger, web, App, HttpServer};
use db::DB_MANAGER;
use dotenvy::dotenv;

mod auth;
mod avatar;
mod db;
mod errors;
mod middleware;
mod role;
mod schema;
mod user;

extern crate diesel;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    DB_MANAGER.lock().unwrap().start_connection().await;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(user::controller::create_user)
            .service(auth::controller::login)
            .service(
                web::scope("")
                    .wrap(middleware::AuthMiddleware)
                    .service(avatar::controller::update_user_avatar)
                    .service(avatar::controller::delete_user_avatar),
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
