use std::{error::Error, fs::File, io::BufReader, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub api_base: String,
    pub model: String,
}

impl Config {
    pub fn init() -> Result<(), Box<dyn Error>> {
        if !Path::new("./config.json").exists() {
            println!("current dir: {:?}", std::env::current_dir());
            return Err("配置文件 config.json 不存在".into());
        }
        // 尝试读取配置文件来验证其内容
        Config::read_config_from_file("config.json")?;
        Ok(())
    }
    pub fn read_config_from_file(file_path: &str) -> Result<Config, Box<dyn Error>> {
        let file = File::open(file_path).expect("Failed to open file");
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}
