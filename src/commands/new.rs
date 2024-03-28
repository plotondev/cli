use std::io;

use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::util::{config_file::Config, http::HttpClient};

/// Create new integration app
#[derive(Parser)]
pub struct Args {}

#[derive(Serialize)]
struct NewIntegrationPayLoad {
    description: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct ConfigResponse {
    id: String,
    name: String,
}
pub async fn command(_args: Args, json: bool) -> Result<()> {
    let mut config = Config::new()?;
    // Implement the new app creation logic here
    // 1) Create a new app
    // 2) Link the app to the current directory
    // 3) Write the config file
    println!("Please enter a small description for the app :");
    let mut app_desc = String::new();
    io::stdin()
        .read_line(&mut app_desc)
        .context("Failed to read line")?;
    let app_desc = app_desc.trim(); // Trim newline and whitespaces
    println!("App description: {}", app_desc);
    let http_client = HttpClient::new(&config);

    let payload = NewIntegrationPayLoad {
        description: app_desc.to_string(),
    };
    let org = config.get_default_org().unwrap_or_default();
    let response = http_client
        .post(format!("/v1/{org}/integrations/cli").as_str(), &payload)
        .await?;
    let ploton_config = response.text().await?;
    let ploton_config_yaml: ConfigResponse = serde_yaml::from_str(&ploton_config)?;
    //write this to a local file called ploton.config
    let current_dir = std::env::current_dir()?;
    let config_path = current_dir.join("ploton.yaml");
    std::fs::write(config_path, ploton_config)?;
    config.link_project(ploton_config_yaml.id, Some(ploton_config_yaml.name))?;
    config.write()?;
    //config.write()?;

    Ok(())
}
