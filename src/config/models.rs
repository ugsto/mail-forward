use super::errors::ConfigLoadError;

#[derive(Debug)]
pub struct Config {
    pub api_address: String,
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub email_from: String,
    pub email_to: String,
}

#[derive(Default, Debug)]
pub struct PartialConfig {
    pub api_address: Option<String>,
    pub smtp_server: Option<String>,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    pub email_from: Option<String>,
    pub email_to: Option<String>,
}

impl TryFrom<PartialConfig> for Config {
    type Error = ConfigLoadError;

    fn try_from(config: PartialConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            api_address: config
                .api_address
                .ok_or(ConfigLoadError::MissingProperty("api_address"))?,
            smtp_server: config
                .smtp_server
                .ok_or(ConfigLoadError::MissingProperty("smtp_server"))?,
            smtp_username: config
                .smtp_username
                .ok_or(ConfigLoadError::MissingProperty("smtp_username"))?,
            smtp_password: config
                .smtp_password
                .ok_or(ConfigLoadError::MissingProperty("smtp_password"))?,
            email_from: config
                .email_from
                .ok_or(ConfigLoadError::MissingProperty("email_from"))?,
            email_to: config
                .email_to
                .ok_or(ConfigLoadError::MissingProperty("email_to"))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn partial_config() -> PartialConfig {
        PartialConfig {
            api_address: Some("http://example.com".to_string()),
            smtp_server: Some("smtp.example.com".to_string()),
            smtp_username: Some("user@example.com".to_string()),
            smtp_password: Some("password".to_string()),
            email_from: Some("from@example.com".to_string()),
            email_to: Some("to@example.com".to_string()),
        }
    }

    #[test]
    fn partial_config_to_config() {
        let partial_config = partial_config();
        let config_result = Config::try_from(partial_config);

        assert!(config_result.is_ok());
        let config = config_result.unwrap();
        assert_eq!(config.api_address, "http://example.com");
        assert_eq!(config.smtp_server, "smtp.example.com");
        assert_eq!(config.smtp_username, "user@example.com");
        assert_eq!(config.smtp_password, "password");
        assert_eq!(config.email_from, "from@example.com");
        assert_eq!(config.email_to, "to@example.com");
    }

    // Tests for each field missing in PartialConfig
    #[test]
    fn missing_api_address() {
        let mut partial_config = partial_config();
        partial_config.api_address = None;
        let config_result = Config::try_from(partial_config);
        assert!(config_result.is_err());
        assert!(matches!(
            config_result.unwrap_err(),
            ConfigLoadError::MissingProperty("api_address")
        ));
    }
}
