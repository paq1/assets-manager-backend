use config::{Config, ConfigError, Environment /*, File*/};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct AuthSettings {
    pub domain: String,
    pub audience: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiSettings {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct MinioSettings {
    pub user: String,
    pub password: String,
}


#[derive(Debug, Deserialize)]
pub struct Settings {
    pub env: String,
    pub api: ApiSettings,
    pub auth: AuthSettings,
    pub minio: MinioSettings,
}

impl Settings {

    pub fn new() -> Result<Self, ConfigError> {

        let config = Config::builder()
            // .add_source(File::with_name("server/config/default.toml"))
            .add_source(
                Environment::default()
                    .prefix("ASSETS_MANAGER")
                    .separator("_")
                    .ignore_empty(true)
                    .try_parsing(true)
            )
            .build()?;

        println!("{:#?}", config.clone().try_deserialize::<serde_json::Value>());

        config.try_deserialize()
    }

}