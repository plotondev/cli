use crate::{
    util::{
        api_key::set_key,
        config_file::{save_config, Config},
        http::HttpClient,
    },
    LOGIN_URL,
};
use anyhow::{Context, Result};
use std::io;

use clap::Parser;

/// Ploton authentication command
#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: Args, json: bool) -> Result<()> {
    println!("Please visit the following URL to obtain your API key:");
    println!("{}/org/api_keys/", LOGIN_URL);
    println!("Paste your API key below:");

    let mut api_key = String::new();
    io::stdin()
        .read_line(&mut api_key)
        .context("Failed to read line")?;
    let api_key = api_key.trim(); // Trim newline and whitespaces

    let response = HttpClient::new().post("/cli", api_key).await?;

    if response.status().is_success() {
        let config: Config = response
            .json()
            .await
            .context("Failed to send validation request")?;

        save_config(&config)?;
        set_key(config.org_id, api_key.to_string()).await?;

        print!(
            "Authenticated as {} for organization: {}\n\n {} set to default organization.\n\n",
            config.user_email, config.org_name, config.org_name
        );

        Ok(())
    } else {
        anyhow::bail!("Failed to validate API key. {}", response.status());
    }
}
