use config::{ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<config::Config, ConfigError> {
        let environment = Environment::default().try_parsing(false);

        let builder = config::Config::builder().add_source(environment);

        builder.build()
    }
}
