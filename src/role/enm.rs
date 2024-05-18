use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoleEnum {
    ADMIN,
    STUDENT,
    TEACHER,
    MONITOR,
}

impl From<RoleEnum> for String {
    fn from(role: RoleEnum) -> Self {
        match role {
            RoleEnum::ADMIN => "ADMIN".to_string(),
            RoleEnum::STUDENT => "STUDENT".to_string(),
            RoleEnum::TEACHER => "TEACHER".to_string(),
            RoleEnum::MONITOR => "MONITOR".to_string(),
        }
    }
}
