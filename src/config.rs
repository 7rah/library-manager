use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub global: Global,
    pub db: Db,
}

#[derive(Debug, Deserialize)]
pub struct Global {
    pub listen_addr: String,
    pub log: String,
}

#[derive(Debug, Deserialize)]
pub struct Db {
    pub addr: String,
    pub log: Option<String>,
}

pub fn init_config() -> Config {
    let mut file = File::open("config.toml").unwrap();
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str).unwrap();
    let config: Config = toml::from_str(&toml_str).unwrap();
    config
}
