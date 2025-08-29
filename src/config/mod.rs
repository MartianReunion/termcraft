mod path_provider;
use figment::{
    providers::{Env, Format, Json},
    Figment,
    Error
};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG_JSON: &str = include_str!("default_config.json");

static CONFIG: OnceCell<AppConfig> = OnceCell::new();

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub debug: bool,
    pub language: String,
}
pub fn init() -> Result<(), Error> {
    let figment = Figment::new()
        .merge(Json::string(include_str!("default_config.json")))
        .merge(if let Some(path) = path_provider::ensure_get_user_config_path() {
            Json::file(path)
        } else {
            Json::string("{}")
        })
        .merge(Env::prefixed("TERMCRAFT_").split("_"));

    CONFIG.set(figment.extract()?).expect("config already initialized");
    Ok(())
}

pub fn get() -> &'static AppConfig {
    CONFIG.get().expect("config not initialized")
}