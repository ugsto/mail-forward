use crate::errors::ServiceError;
use crate::models::ContactForm;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

pub fn send_email(
    form: &ContactForm,
    smtp_server: &str,
    smtp_username: &str,
    smtp_password: &str,
    email_from: &str,
    email_to: &str,
) -> Result<(), ServiceError> {
    let email = Message::builder()
        .from(
            email_from
                .parse()
                .map_err(|_| ServiceError::InternalServerError)?,
        )
        .to(email_to
            .parse()
            .map_err(|_| ServiceError::InternalServerError)?)
        .subject(&form.subject)
        .body(format!(
            "Name: {}\nEmail: {}\nMessage: {}",
            form.name, form.email, form.message
        ))
        .map_err(|_| ServiceError::InternalServerError)?;

    let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

    let mailer = SmtpTransport::relay(smtp_server)
        .map_err(|_| ServiceError::InternalServerError)?
        .credentials(creds)
        .build();

    mailer
        .send(&email)
        .map_err(|_| ServiceError::InternalServerError)?;
    Ok(())
}