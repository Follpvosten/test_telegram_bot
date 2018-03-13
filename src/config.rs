extern crate toml;

use std::fs::File;
use std::io::prelude::Read;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub bot_token: String,
}

pub fn read_config_file(filename: &str) -> Config {
    let mut file = File::open(filename).expect("Error loading config file");
    let mut toml_str = String::new();

    file.read_to_string(&mut toml_str)
        .expect("Error reading file");

    toml::from_str(&toml_str).expect("Bad config file")
}
