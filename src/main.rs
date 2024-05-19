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
                web::scope("/users")
                    .service(web::resource("").post(user::controller::create_user))
                    .service(
                        web::resource("/{user_id}/roles")
                            .wrap(middleware::RoleMiddleware(vec![ADMIN]))
                            .wrap(middleware::AuthMiddleware)
                            .patch(user::controller::set_user_roles),
                    ),
            )
            .service(
                web::scope("/avatars")
                    .wrap(middleware::AuthMiddleware)
                    .service(
                        web::resource("")
                            .post(avatar::controller::update_user_avatar)
                            .delete(avatar::controller::delete_user_avatar),
                    ),
            )
            .service(web::resource("/auth/login").post(auth::controller::login))
            .service(
                web::scope("/classes")
                    .wrap(middleware::AuthMiddleware)
                    .service(
                        web::resource("")
                            .wrap(middleware::RoleMiddleware(vec![TEACHER]))
                            .post(class::controller::create_class),
                    )
                    .service(
                        web::resource("/students/enrolled")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .route(
                                web::get()
                                    .to(class::controller::list_classes_that_student_is_enrolled),
                            ),
                    )
                    .service(
                        web::resource("/students/unenrolled")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .route(
                                web::get().to(
                                    class::controller::list_classes_that_student_is_not_enrolled,
                                ),
                            ),
                    )
                    .service(
                        web::resource("/teachers")
                            .wrap(middleware::RoleMiddleware(vec![TEACHER]))
                            .route(web::get().to(class::controller::list_classes_by_teacher)),
                    )
                    .service(
                        web::resource("/{class_id}")
                            .route(
                                web::get()
                                    .to(class::controller::get_class_by_id)
                                    .wrap(middleware::AuthMiddleware),
                            )
                            .route(
                                web::patch()
                                    .to(class::controller::update_class)
                                    .wrap(middleware::RoleMiddleware(vec![TEACHER])),
                            )
                            .route(
                                web::delete()
                                    .to(class::controller::delete_class)
                                    .wrap(middleware::RoleMiddleware(vec![TEACHER])),
                            ),
                    )
                    .service(
                        web::resource("/{class_id}/exams")
                            .route(web::get().to(exam::controller::list_exams_by_class_id)),
                    )
                    .service(
                        web::resource("/{class_id}/enroll")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .route(web::post().to(class::controller::enroll_student)),
                    ),
            )
            .service(
                web::scope("/questions")
                    .wrap(middleware::RoleMiddleware(vec![TEACHER]))
                    .wrap(middleware::AuthMiddleware)
                    .service(
                        web::resource("")
                            .post(question::controller::create_question)
                            .get(question::controller::list_questions),
                    )
                    .service(
                        web::resource("/{question_id}")
                            .get(question::controller::get_question_by_id)
                            .delete(question::controller::delete_question_by_id)
                            .patch(question::controller::update_question_by_id),
                    )
                    .service(
                        web::resource("/{question_id}/answers")
                            .get(question::controller::list_answers_by_question_id),
                    ),
            )
            .service(
                web::scope("/exams")
                    .wrap(middleware::AuthMiddleware)
                    .service(
                        web::resource("")
                            .wrap(middleware::RoleMiddleware(vec![TEACHER]))
                            .post(exam::controller::create_exam),
                    )
                    .service(
                        web::resource("/{exam_id}")
                            .wrap(middleware::RoleMiddleware(vec![TEACHER]))
                            .get(exam::controller::get_exam_by_id)
                            .delete(exam::controller::delete_exam)
                            .patch(exam::controller::update_exam),
                    )
                    .service(
                        web::resource("/{exam_id}/questions").route(
                            web::post()
                                .to(exam::controller::update_questions_in_exam)
                                .wrap(middleware::RoleMiddleware(vec![TEACHER])),
                        ),
                    )
                    .service(
                        web::resource("/{exam_id}/questions/students")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .get(exam::controller::get_questions_in_exam_as_student),
                    )
                    .service(
                        web::resource("/{exam_id}/questions/teachers")
                            .wrap(middleware::RoleMiddleware(vec![TEACHER]))
                            .get(exam::controller::get_questions_in_exam_as_teacher),
                    )
                    .service(
                        web::resource("/{exam_id}/question/{question_id}/submit")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .post(exam::controller::submit_answer_to_question_in_exam),
                    )
                    .service(
                        web::resource("/{exam_id}/results")
                            .wrap(middleware::RoleMiddleware(vec![TEACHER]))
                            .get(exam::controller::get_exam_results_as_teacher),
                    )
                    .service(
                        web::resource("/{exam_id}/results/students")
                            .wrap(middleware::RoleMiddleware(vec![STUDENT]))
                            .get(exam::controller::get_exam_results_as_student),
                    ),
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
