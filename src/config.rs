use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

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

const EXAMPLE_CONFIG: &str = r#"[global]
listen_addr = "127.0.0.1:3000"
log = "info"

[db]
addr = "sqlite://:memory:"
#addr = "mysql://root:123456@localhost:3306/books_manager"
log = "warn"
"#;

pub fn init_config() -> Config {
    let path = "config.toml";
    if !Path::new(path).exists() {
        let mut file = File::create(path).unwrap();
        file.write_all(EXAMPLE_CONFIG.as_bytes().as_ref()).unwrap();
    }

    let mut file = File::open(path).unwrap();
    let mut toml_str = String::new();
    file.read_to_string(&mut toml_str).unwrap();
    let config: Config = toml::from_str(&toml_str).unwrap();
    config
}
