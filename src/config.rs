use std::{error::Error, fs::File, io::BufReader};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub api_base: String,
    pub model: String,
}

impl Config {
    pub fn read_config_from_file(file_path: &str) -> Result<Config, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}
