use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
    pub bearer_token: String,
}

impl Config {
   pub fn from_env() -> Result<Config, Box<dyn Error>> {
        let consumer_key = env::var("TWT_API_KEY_PIRATE")?;
        let consumer_secret = env::var("TWT_API_SECRET_PIRATE")?;
        let access_key = env::var("TWT_ACCESS_TOKEN_PIRATE")?;
        let access_secret = env::var("TWT_ACCESS_TOKEN_SECRET_PIRATE")?;
        let bearer_token = env::var("TWT_BEARER_TOKEN")?;
        
        Ok(Config { 
            consumer_key,
            consumer_secret,
            access_key,
            access_secret,
            bearer_token,
        })
    }
}
