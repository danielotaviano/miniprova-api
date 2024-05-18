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

impl CreateAnswerInputDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.answer.is_empty() {
            return Err("Answer is required".to_string());
        }

        Ok(())
    }
}
