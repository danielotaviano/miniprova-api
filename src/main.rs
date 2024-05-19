use actix_web::{middleware::Logger, web, App, HttpServer};
use db::DB_MANAGER;
use dotenvy::dotenv;
use role::enm::RoleEnum::*;

mod auth;
mod avatar;
mod class;
mod db;
mod errors;
mod exam;
mod middleware;
mod question;
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
            .service(
                web::resource("/class")
                    .wrap(middleware::RoleMiddleware(TEACHER))
                    .wrap(middleware::AuthMiddleware)
                    .post(class::controller::create_class),
            )
            .service(
                web::resource("/class/student/enrolled")
                    .wrap(middleware::RoleMiddleware(STUDENT))
                    .wrap(middleware::AuthMiddleware)
                    .route(web::get().to(class::controller::list_classes_that_student_is_enrolled)),
            )
            .service(
                web::resource("/class/student/unenrolled")
                    .wrap(middleware::RoleMiddleware(STUDENT))
                    .wrap(middleware::AuthMiddleware)
                    .route(
                        web::get().to(class::controller::list_classes_that_student_is_not_enrolled),
                    ),
            )
            .service(
                web::resource("/class/{class_id}")
                    .route(
                        web::get()
                            .to(class::controller::get_class_by_id)
                            .wrap(middleware::AuthMiddleware),
                    )
                    .route(
                        web::patch()
                            .to(class::controller::update_class)
                            .wrap(middleware::RoleMiddleware(TEACHER))
                            .wrap(middleware::AuthMiddleware),
                    )
                    .route(
                        web::delete()
                            .to(class::controller::delete_class)
                            .wrap(middleware::RoleMiddleware(TEACHER))
                            .wrap(middleware::AuthMiddleware),
                    ),
            )
            .service(
                web::resource("/class/{class_id}/exams")
                    .wrap(middleware::RoleMiddleware(TEACHER))
                    .wrap(middleware::AuthMiddleware)
                    .route(web::get().to(exam::controller::list_exams_by_class_id)),
            )
            .service(
                web::resource("/class/{class_id}/enroll")
                    .wrap(middleware::RoleMiddleware(STUDENT))
                    .wrap(middleware::AuthMiddleware)
                    .route(web::post().to(class::controller::enroll_student)),
            )
            .service(
                web::resource("/class/teacher/list")
                    .wrap(middleware::RoleMiddleware(TEACHER))
                    .wrap(middleware::AuthMiddleware)
                    .route(web::get().to(class::controller::list_classes_by_teacher)),
            )
            .service(
                web::scope("/question")
                    .wrap(middleware::RoleMiddleware(TEACHER))
                    .wrap(middleware::AuthMiddleware)
                    .service(
                        web::resource("")
                            .post(question::controller::create_question)
                            .get(question::controller::list_questions),
                    )
                    .service(
                        web::resource("/{question_id}")
                            .get(question::controller::get_question_by_id)
                            .delete(question::controller::delete_question_by_id),
                    )
                    .service(
                        web::resource("/{question_id}/answers")
                            .get(question::controller::list_answers_by_question_id),
                    ),
            )
            .service(
                web::scope("/exam")
                    .wrap(middleware::RoleMiddleware(TEACHER))
                    .wrap(middleware::AuthMiddleware)
                    .service(web::resource("").post(exam::controller::create_exam))
                    .service(
                        web::resource("/{exam_id}")
                            .get(exam::controller::get_exam_by_id)
                            .delete(exam::controller::delete_exam)
                            .patch(exam::controller::update_exam),
                    ), // .service(
                       //     web::resource("/{exam_id}/questions")
                       //         .get(exam::controller::list_questions_by_exam_id),
                       // )
                       // .service(
                       //     web::resource("/{exam_id}/questions/{question_id}")
                       //         .post(exam::controller::add_question_to_exam)
                       //         .delete(exam::controller::remove_question_from_exam),
                       // ),
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
