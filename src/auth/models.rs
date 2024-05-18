use serde::{Deserialize, Serialize};

use crate::role::enm::RoleEnum;

#[derive(Debug, Serialize, Deserialize)]

pub struct LoggedUser {
    pub id: i32,
    pub roles: Vec<RoleEnum>,
}
