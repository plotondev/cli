use anyhow::Result;
use dirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs, io};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub org_id: String,
    pub org_name: String,
    pub user_email: String,
}

pub fn save_config(config: &Config) -> Result<(), io::Error> {
    // Use `dirs` crate to find the home directory
    let home_dir = dirs::home_dir().ok_or(io::Error::new(
        io::ErrorKind::NotFound,
        "Home directory not found",
    ))?;
    let ploton_dir = home_dir.join(".ploton");

    // Create `.ploton` directory if it doesn't exist
    fs::create_dir_all(&ploton_dir)?;

    let config_path = ploton_dir.join("config.json");
    let config_data = serde_json::to_string_pretty(&config)?;

    fs::write(config_path, config_data)?;
    Ok(())
}
