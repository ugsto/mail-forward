use actix_web::{error::InternalError, http::StatusCode, web, App, HttpResponse, HttpServer};
use lazy_static::lazy_static;
use std::process::exit;
use validator::Validate;

mod config;
mod email_service;
mod errors;
mod models;

use config::{
    models::Config, strategies::env_config_loader::EnvConfigLoader, traits::PartialConfigLoader,
};
use errors::ServiceError;
use models::ContactForm;

lazy_static! {
    static ref CONFIG: Config =
        Config::try_from(EnvConfigLoader::load_partial_config().unwrap_or_else(|e| {
            eprintln!("Failed to load configuration: {}", e);
            exit(1)
        }))
        .unwrap_or_else(|e| {
            eprintln!("Failed to parse configuration: {}", e);
            exit(1)
        });
}

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
    println!(
        "Starting server on host: {}; port: {}",
        CONFIG.api_host, CONFIG.api_port
    );
    HttpServer::new(|| App::new().route("/submit_form", web::post().to(submit_contact_form)))
        .bind(format!("{}:{}", CONFIG.api_host, CONFIG.api_port))?
        .run()
        .await
}
