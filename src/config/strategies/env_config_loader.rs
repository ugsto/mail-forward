use crate::config::{
    errors::PartialConfigLoadError, models::PartialConfig, traits::PartialConfigLoader,
};
use dotenv::dotenv;
use std::env;

pub struct EnvConfigLoader;

impl PartialConfigLoader for EnvConfigLoader {
    fn load_partial_config() -> Result<PartialConfig, PartialConfigLoadError> {
        dotenv().ok();

        let api_address = env::var("API_ADDRESS").ok();
        let smtp_server = env::var("SMTP_SERVER").ok();
        let smtp_username = env::var("SMTP_USERNAME").ok();
        let smtp_password = env::var("SMTP_PASSWORD").ok();
        let email_from = env::var("EMAIL_FROM").ok();
        let email_to = env::var("EMAIL_TO").ok();

        let partial_config = PartialConfig {
            api_address,
            smtp_server,
            smtp_username,
            smtp_password,
            email_from,
            email_to,
        };

        Ok(partial_config)
    }
}
