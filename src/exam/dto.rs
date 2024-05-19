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
