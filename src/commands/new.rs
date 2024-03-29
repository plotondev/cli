use std::io;

use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::util::{config_file::Config, http::HttpClient, local_file::LocalFile};

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
pub async fn command(_args: Args, _: bool) -> Result<()> {
    let mut local_file = LocalFile::new()?;

    let mut config = Config::new()?;
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
    if response.status().is_success() {
        let ploton_config = response.text().await?;
        let ploton_config_yaml: LocalFile = serde_yaml::from_str(&ploton_config)?;

        local_file.load(ploton_config_yaml.clone());
        local_file.write()?;

        config.link_project(ploton_config_yaml.id, Some(ploton_config_yaml.name))?;
        config.write()?;

        Ok(())
    } else {
        let error = response.text().await?;
        println!("Error: {}", error);
        Ok(())
    }
}
