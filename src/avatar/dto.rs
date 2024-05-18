use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAvatarInputDto {
    pub url: String,
}

impl CreateAvatarInputDto {
    pub fn validate(&self) -> Result<(), String> {
        if self.url.is_empty() {
            return Err("Url is required".into());
        }

        Ok(())
    }
}
