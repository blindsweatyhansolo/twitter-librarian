use log::info;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
}

impl Config {
    pub fn read(path_file: &Path) -> Config {
        let mut file = File::open(path_file).expect("Failed to open!");

        serde_json::from_reader(&mut file).ok().unwrap()
    }

    pub fn from_env() -> Result<Config, Box<dyn Error>> {
        info!("Loading secrets from system environment...");

        let consumer_key = env::var("TWT_API_KEY_PIRATE")?;
        let consumer_secret = env::var("TWT_API_SECRET_PIRATE")?;
        let access_key = env::var("TWT_ACCESS_TOKEN_PIRATE")?;
        let access_secret = env::var("TWT_ACCESS_TOKEN_SECRET_PIRATE")?;

        Ok(Config {
            consumer_key,
            consumer_secret,
            access_key,
            access_secret,
        })
    }
}
