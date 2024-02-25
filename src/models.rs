use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct ContactForm {
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub subject: String,
    pub message: String,
}
