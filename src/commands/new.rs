use std::io;

use anyhow::{Context, Result};
use clap::Parser;
use serde::Serialize;

use crate::util::{config_file::Config, http::HttpClient};

/// Create new integration app
#[derive(Parser)]
pub struct Args {}

#[derive(Serialize)]
struct NewIntegrationPayLoad {
    desc: String,
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
        desc: app_desc.to_string(),
    };
    let org = config.get_default_org().unwrap_or_default();
    let response = http_client
        .post(format!("/v1/{org}/integrations/cli").as_str(), &payload)
        .await?;
    println!("{:?}", response);
    //config.write()?;

    Ok(())
}
