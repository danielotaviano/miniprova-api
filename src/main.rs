use actix_web::{middleware::Logger, web, App, HttpServer};
use db::DB_MANAGER;
use dotenvy::dotenv;
use role::enm::RoleEnum::*;

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
            .service(
                web::scope("/user")
                    .service(web::resource("").post(user::controller::create_user))
                    .service(
                        web::resource("/{user_id}/roles")
                            .wrap(middleware::RoleMiddleware(ADMIN))
                            .wrap(middleware::AuthMiddleware)
                            .patch(user::controller::set_user_roles),
                    ),
            )
            .service(
                web::scope("/avatar")
                    .wrap(middleware::AuthMiddleware)
                    .service(
                        web::resource("")
                            .post(avatar::controller::update_user_avatar)
                            .delete(avatar::controller::delete_user_avatar),
                    ),
            )
            .service(web::resource("/login").post(auth::controller::login))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
