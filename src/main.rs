use actix_web::{error::InternalError, http::StatusCode, web, App, HttpResponse, HttpServer};
use config::CONFIG;
use std::env;
use validator::Validate;

mod config;
mod email_service;
mod errors;
mod models;

use errors::ServiceError;
use models::ContactForm;

async fn submit_contact_form(
    form: web::Form<ContactForm>,
) -> Result<HttpResponse, InternalError<ServiceError>> {
    form.validate().map_err(|e| {
        InternalError::new(
            ServiceError::ValidationError(e.to_string()),
            StatusCode::BAD_REQUEST,
        )
    })?;

    email_service::send_email(
        &form,
        &CONFIG.smtp_server,
        &CONFIG.smtp_username,
        &CONFIG.smtp_password,
        &CONFIG.email_from,
        &CONFIG.email_to,
    )
    .map(|_| HttpResponse::Ok().body("Form submitted successfully!"))
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/submit_form", web::post().to(submit_contact_form)))
        .bind(format!(
            "{}:{}",
            env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            env::var("PORT").unwrap_or_else(|_| "8080".to_string())
        ))?
        .run()
        .await
}
