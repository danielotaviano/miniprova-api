use std::error::Error;

use diesel::RunQueryDsl;

use crate::{
    db::DB_MANAGER,
    errors::ServiceError,
    question::{
        dto::QuestionWithAnswersDto,
        models::{Answer, Question},
    },
    schema::{answers, exam_questions, exams, questions, student_answers},
};

use super::{
    dto::StudentExamResultDto,
    models::{Exam, NewExam, UpdateExam},
};
use crate::diesel::*;

pub fn create_exam(new_exam: NewExam) -> Result<Exam, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result: Exam = diesel::insert_into(exams::table)
        .values(new_exam)
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(result)
}

pub fn get_exam_by_id(exam_id: i32) -> Result<Option<Exam>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let exam = exams::table
        .find(exam_id)
        .first(&mut conn)
        .optional()
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(exam)
}

pub fn get_exams_by_class_id(class_id: i32) -> Result<Vec<Exam>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let exams = exams::table
        .filter(exams::class_id.eq(class_id))
        .load(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(exams)
}

pub fn update_exam(exam_id: i32, new_exam: UpdateExam) -> Result<Exam, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result: Exam = diesel::update(exams::table.find(exam_id))
        .set(new_exam)
        .get_result(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(result)
}

pub fn delete_exam(exam_id: i32) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    diesel::delete(exams::table.find(exam_id))
        .execute(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(())
}

pub fn update_questions_in_exam(exam_id: i32, question_ids: Vec<i32>) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let result = conn.transaction::<_, Box<dyn Error>, _>(|tx| {
        diesel::delete(exam_questions::table.filter(exam_questions::exam_id.eq(exam_id)))
            .execute(tx)?;

        for question_id in question_ids {
            diesel::insert_into(exam_questions::table)
                .values((
                    exam_questions::exam_id.eq(exam_id),
                    exam_questions::question_id.eq(question_id),
                ))
                .execute(tx)?;
        }

        Ok(())
    });

    if result.is_err() {
        println!("{:?}", result.err().unwrap());
        return Err(ServiceError::InternalServerError);
    }

    Ok(())
}

pub fn get_questions_in_exam(exam_id: i32) -> Result<Vec<QuestionWithAnswersDto>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let questions: Vec<Question> = exam_questions::table
        .inner_join(questions::table)
        .filter(exam_questions::exam_id.eq(exam_id))
        .select(questions::all_columns)
        .load(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    let mut result = Vec::new();

    for question in questions {
        let answers: Vec<Answer> = answers::table
            .filter(answers::question_id.eq(question.id))
            .load(&mut conn)
            .map_err(|_| ServiceError::InternalServerError)?;

        let answers_dto = answers
            .iter()
            .map(|answer| crate::question::dto::AnswerDto {
                id: answer.id,
                answer: answer.answer.clone(),
                is_correct: Some(answer.is_correct),
            })
            .collect();

        result.push(QuestionWithAnswersDto {
            id: question.id,
            question: question.question,
            answers: answers_dto,
        });
    }

    Ok(result)
}

pub fn submit_answer_to_question_in_exam(
    exam_id: i32,
    question_id: i32,
    student_id: i32,
    answer_id: i32,
) -> Result<(), ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    println!(
        "submit_answer_to_question_in_exam: {}, {}, {}, {}",
        exam_id, question_id, student_id, answer_id
    );

    diesel::insert_into(student_answers::table)
        .values((
            student_answers::user_id.eq(student_id),
            student_answers::exam_id.eq(exam_id),
            student_answers::question_id.eq(question_id),
            student_answers::answer_id.eq(answer_id),
        ))
        .on_conflict((
            student_answers::user_id,
            student_answers::exam_id,
            student_answers::question_id,
        ))
        .do_update()
        .set(student_answers::answer_id.eq(answer_id))
        .execute(&mut conn)
        .map_err(|e| {
            println!("{:?}", e);
            ServiceError::InternalServerError
        })?;

    Ok(())
}

pub fn get_exam_results_as_student(
    exam_id: i32,
    student_id: i32,
) -> Result<StudentExamResultDto, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let questions: Vec<Question> = exam_questions::table
        .inner_join(questions::table)
        .filter(exam_questions::exam_id.eq(exam_id))
        .select(questions::all_columns)
        .load(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    let mut student_answer_results = Vec::new();

    for question in questions {
        let student_answer: Option<i32> = student_answers::table
            .filter(
                student_answers::user_id
                    .eq(student_id)
                    .and(student_answers::exam_id.eq(exam_id))
                    .and(student_answers::question_id.eq(question.id)),
            )
            .select(student_answers::answer_id)
            .first(&mut conn)
            .optional()
            .map_err(|_| ServiceError::InternalServerError)?;

        let is_correct = match student_answer {
            Some(student_answer) => {
                let answer = answers::table
                    .find(student_answer)
                    .first::<Answer>(&mut conn)
                    .map_err(|_| ServiceError::InternalServerError)?;

                answer.is_correct
            }
            None => None::<bool>.is_some(),
        };

        student_answer_results.push(crate::exam::dto::StudentExamAnswerResultDto {
            question_id: question.id,
            answer_id: student_answer.unwrap_or(0),
            is_correct,
        });
    }

    let score = student_answer_results
        .iter()
        .filter(|result| result.is_correct)
        .count() as f32
        / student_answer_results.len() as f32;

    let student = crate::user::service::get_user_with_roles_by_id(student_id)?;

    if student.is_none() {
        return Err(ServiceError::BadRequest("Student not found".to_string()));
    }

    let student = student.unwrap();

    Ok(crate::exam::dto::StudentExamResultDto {
        score,
        student_answer_results,
        id: student.id,
        name: student.name,
    })
}

pub fn get_exam_results_as_teacher(
    exam_id: i32,
) -> Result<Vec<StudentExamResultDto>, ServiceError> {
    let mut conn = DB_MANAGER.lock().unwrap().get_database();

    let student_ids: Vec<i32> = student_answers::table
        .filter(student_answers::exam_id.eq(exam_id))
        .select(student_answers::user_id)
        .distinct()
        .load(&mut conn)
        .map_err(|_| ServiceError::InternalServerError)?;

    let mut results = Vec::new();

    for student_id in student_ids {
        let result = get_exam_results_as_student(exam_id, student_id)?;

        results.push(result);
    }

    Ok(results)
}
