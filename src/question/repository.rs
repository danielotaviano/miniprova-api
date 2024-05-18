use crate::diesel::OptionalExtension;
use std::error::Error;

use diesel::{query_dsl::methods::FilterDsl, Connection, ExpressionMethods, RunQueryDsl};

use crate::{
    db::DB_MANAGER,
    errors::ServiceError,
    question::models::Answer,
    schema::{answers, questions},
};

use super::{
    dto::CreateQuestionInputDto,
    models::{NewAnswer, NewQuestion, Question},
};

pub fn create_question(new_question: CreateQuestionInputDto) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result: Result<(), Box<dyn Error>> = conn.transaction(|tx| {
        let question = diesel::insert_into(questions::table)
            .values(&NewQuestion {
                question: &new_question.question,
            })
            .returning(questions::all_columns)
            .get_result::<Question>(tx)?;

        for answer in new_question.answers {
            let _: Answer = diesel::insert_into(answers::table)
                .values(&NewAnswer {
                    answer: answer.answer,
                    is_correct: answer.is_correct,
                    question_id: question.id,
                })
                .returning(answers::all_columns)
                .get_result(tx)?;
        }

        Ok(())
    });

    if result.is_err() {
        println!("{:?}", result.err().unwrap());
        return Err(ServiceError::InternalServerError);
    }

    Ok(())
}

pub fn get_question_by_id(question_id: i32) -> Result<Option<Question>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let question = questions::table
        .filter(questions::id.eq(question_id))
        .first::<Question>(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(question)
}

pub fn delete_question_by_id(question_id: i32) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    diesel::delete(answers::table.filter(answers::question_id.eq(question_id)))
        .execute(&mut conn)
        .map_err(|e| {
            println!("{:?}", e);
            ServiceError::InternalServerError
        })?;

    diesel::delete(questions::table.filter(questions::id.eq(question_id)))
        .execute(&mut conn)
        .map_err(|e| {
            println!("{:?}", e);
            ServiceError::InternalServerError
        })?;

    Ok(())
}

pub fn update_question(
    question_id: i32,
    new_question: CreateQuestionInputDto,
) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result: Result<(), Box<dyn Error>> = conn.transaction(|tx| {
        diesel::delete(answers::table.filter(answers::question_id.eq(question_id))).execute(tx)?;

        diesel::update(questions::table.filter(questions::id.eq(question_id)))
            .set(questions::question.eq(&new_question.question))
            .execute(tx)?;

        for answer in new_question.answers {
            diesel::insert_into(answers::table)
                .values(&NewAnswer {
                    answer: answer.answer,
                    is_correct: answer.is_correct,
                    question_id,
                })
                .execute(tx)?;
        }

        Ok(())
    });

    if result.is_err() {
        return Err(ServiceError::InternalServerError);
    }

    Ok(())
}

pub fn list_answers_by_question_id(question_id: i32) -> Result<Vec<Answer>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let answers = answers::table
        .filter(answers::question_id.eq(question_id))
        .load::<Answer>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(answers)
}

pub fn list_questions() -> Result<Vec<Question>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();
    let questions = questions::table
        .load::<Question>(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(questions)
}
