use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{
    fs::File,
    io::{Read, Write},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub admin_id: String,
    pub bot_token: String,
    pub debug: bool,
}

pub fn init_default_config() {
    let default_config =
        Config { admin_id: String::from(""), bot_token: String::from(""), debug: true };

    save_config(&default_config).unwrap()
}

pub fn save_config(config: &Config) -> Result<()> {
    let json = serde_json::to_string_pretty(config)?;
    let mut file = File::create("config.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
    Ok(())
}

pub fn load_config() -> Result<Config> {
    let mut file = File::open("config.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}
