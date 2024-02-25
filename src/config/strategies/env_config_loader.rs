use crate::config::{
    errors::PartialConfigLoadError, models::PartialConfig, traits::PartialConfigLoader,
};
use dotenv::dotenv;
use std::env;

pub struct EnvConfigLoader;

impl PartialConfigLoader for EnvConfigLoader {
    fn load_partial_config() -> Result<PartialConfig, PartialConfigLoadError> {
        dotenv().ok();

        let api_host = env::var("HOST").ok();
        let api_port = env::var("PORT").ok().map(|p| p.parse::<u16>().unwrap());
        let smtp_server = env::var("SMTP_SERVER").ok();
        let smtp_username = env::var("SMTP_USERNAME").ok();
        let smtp_password = env::var("SMTP_PASSWORD").ok();
        let email_from = env::var("EMAIL_FROM").ok();
        let email_to = env::var("EMAIL_TO").ok();

        let partial_config = PartialConfig {
            api_host,
            api_port,
            smtp_server,
            smtp_username,
            smtp_password,
            email_from,
            email_to,
        };

        Ok(partial_config)
    }
}
