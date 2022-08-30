use log::info;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub TWT_API_KEY_PIRATE: String,
    pub TWT_API_SECRET_PIRATE: String,
    pub TWT_BEARER_TOKEN_PIRATE: String,
    pub TWT_ACCESS_TOKEN_PIRATE: String,
    pub TWT_ACCESS_TOKEN_SECRET_PIRATE: String,
}

impl Config {
    pub fn read(path_file: &Path) -> Config {
        let mut file = File::open(path_file).expect("Failed to open!");

        serde_derive::from_reader(&mut file).ok().unwrap()
    }

    pub fn from_env() -> Result<Config, Box<dyn Error>> {
        info!("Loading secrets from system environment...");

        let TWT_API_KEY_PIRATE = env::var("TWT_API_KEY_PIRATE")?;
        let TWT_API_SECRET_PIRATE = env::var("TWT_API_SECRET_PIRATE")?;
        let TWT_BEARER_TOKEN_PIRATE = env::var("TWT_BEARER_TOKEN_PIRATE")?;
        let TWT_ACCESS_TOKEN_PIRATE = env::var("TWT_ACCESS_TOKEN_PIRATE")?;
        let TWT_ACCESS_TOKEN_SECRET_PIRATE = env::var("TWT_ACCESS_TOKEN_SECRET_PIRATE")?;

        Ok(Config {
            TWT_API_KEY_PIRATE,
            TWT_API_SECRET_PIRATE,
            TWT_BEARER_TOKEN_PIRATE,
            TWT_ACCESS_TOKEN_PIRATE,
            TWT_ACCESS_TOKEN_SECRET_PIRATE,
        })
    }
}