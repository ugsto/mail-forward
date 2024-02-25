use lazy_static::lazy_static;
use std::env;

pub struct Config {
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub email_from: String,
    pub email_to: String,
}

lazy_static! {
    pub static ref CONFIG: Config = {
        dotenv::dotenv().ok();

        Config {
            smtp_server: env::var("SMTP_SERVER").expect("SMTP_SERVER must be set"),
            smtp_username: env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set"),
            smtp_password: env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
            email_from: env::var("EMAIL_FROM").expect("EMAIL_FROM must be set"),
            email_to: env::var("EMAIL_TO").expect("EMAIL_TO must be set"),
        }
    };
}
