use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateQuestionInputDto {
    pub question: String,
    pub answers: Vec<CreateAnswerInputDto>,
}

impl CreateQuestionInputDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.answers.is_empty() {
            return Err("Must have at least 1 answer".to_string());
        }

        if self.question.is_empty() {
            return Err("Question is required".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateAnswerInputDto {
    pub answer: String,
    pub is_correct: bool,
}

#[derive(Serialize)]
pub struct QuestionWithAnswersDto {
    pub id: i32,
    pub question: String,
    pub answers: Vec<AnswerDto>,
}

#[derive(Serialize)]
pub struct AnswerDto {
    pub id: i32,
    pub answer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_correct: Option<bool>,
}
