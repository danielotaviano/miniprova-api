use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateExamInputDto {
    pub name: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub class_id: i32,
}

impl CreateExamInputDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name is required".to_string());
        }

        if self.start_date >= self.end_date {
            return Err("Start date must be before end date".to_string());
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct StudentAnswerInputDto {
    pub answer_id: i32,
}

impl StudentAnswerInputDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.answer_id <= 0 {
            return Err("Answer id is required".to_string());
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct StudentExamResultDto {
    pub id: i32,
    pub name: String,
    pub score: f32,
    pub student_answer_results: Vec<StudentExamAnswerResultDto>,
}
#[derive(Serialize, Deserialize)]
pub struct StudentExamAnswerResultDto {
    pub question_id: i32,
    pub answer_id: i32,
    pub is_correct: bool,
}
