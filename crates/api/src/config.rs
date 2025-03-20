use std::{
    fs::{read_to_string, File},
    io::Write as _,
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::error::{StartUpError, StartUpResult};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub server: Server,
    pub logging: Logging,
    #[serde(rename = "tv-db")]
    pub tv_db: TvDB,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TvDB {
    pub pin: Option<String>,
    pub key: String,
}

impl Default for TvDB {
    fn default() -> Self {
        Self {
            pin: None,
            key: "82945bab-429b-45cf-ab96-3a085a561599".to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Logging {
    pub level: String,
}

impl Default for Logging {
    fn default() -> Self {
        Logging {
            level: "info".to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub https: Option<u16>,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            host: "0.0.0.0".to_string(),
            port: 8080,
            https: None,
        }
    }
}

pub fn get_config(path: &Path) -> StartUpResult<Config> {
    if !path.exists() {
        log::info!("Config file not found. Creating default Config.toml...");
        let c = Config::default();
        let toml_str = toml::to_string_pretty(&c).map_err(StartUpError::DisplayConfig)?;
        File::create(path)
            .map_err(StartUpError::CreateConfig)?
            .write_all(toml_str.as_bytes())
            .map_err(StartUpError::CreateConfig)?;
        Ok(c)
    } else {
        toml::from_str(&read_to_string(path).map_err(StartUpError::ReadConfig)?)
            .map_err(StartUpError::ParseConfig)
    }
}
