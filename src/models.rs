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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email_format() {
        let form = ContactForm {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            subject: "Test Subject".to_string(),
            message: "Test Message".to_string(),
        };

        assert!(form.validate().is_ok());
    }

    #[test]
    fn test_invalid_email_format() {
        let invalid_form = ContactForm {
            name: "Jane Doe".to_string(),
            email: "invalid_email".to_string(),
            subject: "Test Subject".to_string(),
            message: "Test Message".to_string(),
        };

        assert!(invalid_form.validate().is_err());
    }
}
