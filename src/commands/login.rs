use anyhow::{Context, Result};
use dirs;
use keyring::Entry;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs, io};

#[derive(Serialize)]
struct ApiKeyPayload {
    api_key: String,
}

#[derive(Serialize, Deserialize)]
struct Config {
    org_id: String,
    org_name: String,
    user_email: String,
}

fn save_config(config: &Config) -> Result<(), io::Error> {
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

pub async fn execute() -> Result<()> {
    println!("Please visit the following URL to obtain your API key:");
    println!("https://714844344.propelauthtest.com/org/api_keys/");
    println!("Enter your API key:");

    let mut api_key = String::new();
    io::stdin()
        .read_line(&mut api_key)
        .context("Failed to read line")?;
    let api_key = api_key.trim(); // Trim newline and whitespaces

    // Perform API key validation
    let client = reqwest::Client::new();

    let payload = ApiKeyPayload {
        api_key: api_key.to_string(),
    };

    let response = client
        .post("http://localhost:9090/cli")
        .json(&payload)
        .send()
        .await
        .context("Failed to send validation request")?;

    if response.status().is_success() {
        let config: Config = response
            .json()
            .await
            .context("Failed to send validation request")?;

        save_config(&config)?;

        let keyring = Entry::new("ploton", &config.org_id)?;
        keyring
            .set_password(api_key)
            .context("Failed to store API key in keyring")?;

        print!(
            "Authentication as user: {} for organization: {}\n",
            config.user_email, config.org_name
        );

        Ok(())
    } else {
        anyhow::bail!(
            "Failed to validate API key. HTTP Error: {}",
            response.status()
        );
    }
}
