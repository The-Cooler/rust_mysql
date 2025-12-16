use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use dirs::config_dir;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new(host: String, port: u16, username: String, password: String) -> Self {
        Config {
            host,
            port,
            username,
            password,
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let mut path = config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("rust_mysql");
    fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}

pub fn save_config(config: &Config) -> Result<()> {
    let path = get_config_path();
    let json = serde_json::to_string_pretty(config)?;
    fs::write(&path, json)?;
    Ok(())
}

pub fn load_config() -> Result<Config> {
    let path = get_config_path();
    if !path.exists() {
        return Ok(Config::new(
            String::from("localhost"),
            3306,
            String::from("root"),
            String::new(),
        ));
    }
    let content = fs::read_to_string(&path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

