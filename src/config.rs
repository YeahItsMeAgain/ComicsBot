use config_file::FromConfigFile;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub bot_token: String,
    pub database_url: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::from_config_file("config.json").unwrap());
