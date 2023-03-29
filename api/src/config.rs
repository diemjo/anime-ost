use std::path::PathBuf;

use directories::BaseDirs;
use figment::Figment;
use figment::providers::{Format, Serialized, Yaml, Env};
use serde_derive::{Deserialize,Serialize};

use crate::error::Error;
use crate::result::Result;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AppConfig {
    pub server_port: u16,
    pub proxer_username: String,
    pub proxer_password: String,
    pub proxer_users: Vec<u32>,
    pub db_path: PathBuf,
}

impl AppConfig {

    pub fn load_config() -> Result<Self> {
        Ok(Figment::from(Serialized::defaults(AppConfig::default()))
            .merge(Yaml::file("/etc/anime-ost/config.yaml"))
            .merge(Yaml::file(BaseDirs::new().expect("No Config Directory found").config_dir().join("anime-ost").join("config.yaml").as_path()))
            .merge(Yaml::file("/config/config.yaml"))
            .merge(Yaml::file("./config.yaml"))
            .merge(Env::prefixed("ANIME_OST_"))
            .extract()
            .or_else(|e| Err(Error::FigmentError(e)))?
        )
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            server_port: 8000,
            proxer_username: "user".to_string(),
            proxer_password: "password".to_string(),
            proxer_users: vec![],
            db_path: PathBuf::from("./ost.db")
        }
    }
}