extern crate toml;

use std::fs::File;
use std::io::prelude::Read;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bot_token: String,
}

#[derive(Debug)]
pub enum ConfigReadError {
    IOError(::std::io::Error),
    DeserializeError(toml::de::Error),
}

pub type ConfigReadResult = Result<Config, ConfigReadError>;

pub fn read_config_file(filename: &str) -> ConfigReadResult {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(ConfigReadError::IOError(e)),
    };

    let mut toml_str = String::new();
    match file.read_to_string(&mut toml_str) {
        Ok(_) => {}
        Err(e) => return Err(ConfigReadError::IOError(e)),
    };

    match toml::from_str::<Config>(&toml_str) {
        Ok(config) => Ok(config),
        Err(e) => Err(ConfigReadError::DeserializeError(e)),
    }
}
