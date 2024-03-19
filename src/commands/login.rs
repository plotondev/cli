use crate::{
    util::{api_key::set_key, config_file::Config, http::HttpClient},
    LOGIN_URL,
};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::io;

use clap::Parser;

/// Ploton authentication command
#[derive(Parser)]
pub struct Args {}

#[derive(Deserialize)]
struct LoginResp {
    org_id: String,
    org_name: String,
    user_email: String,
}
pub async fn command(_args: Args, _: bool) -> Result<()> {
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
        let resp: LoginResp = response
            .json()
            .await
            .context("Failed to send validation request")?;

        let mut config = Config::new()?;
        config.set_user(api_key.to_string(), resp.org_id.clone());
        config.set_default_org(resp.org_id.clone());
        config.write()?;

        print!(
            "Authenticated as {} for organization: {}\n\n {} set to default organization.\n\n",
            resp.user_email, resp.org_name, resp.org_name
        );

        Ok(())
    } else {
        anyhow::bail!("Failed to validate API key. {}", response.status());
    }
}
