use crate::{
    util::{config_file::Config, http::HttpClient},
    LOGIN_URL, TICK_STRING,
};
use indicatif::{ProgressBar, ProgressStyle};
use is_terminal::IsTerminal;

use anyhow::{Context, Result};
use serde::Deserialize;
use std::{io, time::Duration};

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

pub async fn command(_args: Args) -> Result<()> {
    println!("Please visit the following URL to obtain your API key:");
    println!("{}/org/api_keys/", LOGIN_URL);
    println!("Paste your API key below:");

    let mut api_key = String::new();
    io::stdin()
        .read_line(&mut api_key)
        .context("Failed to read line")?;
    let api_key = api_key.trim(); // Trim newline and whitespaces

    let spinner = if std::io::stdout().is_terminal() {
        let spinner = ProgressBar::new_spinner()
            .with_style(
                ProgressStyle::default_spinner()
                    .tick_chars(TICK_STRING)
                    .template("{spinner:.green} {msg:.cyan.bold}")?,
            )
            .with_message("Authenticating".to_string());
        spinner.enable_steady_tick(Duration::from_millis(100));
        Some(spinner)
    } else {
        println!("Authenticating...");
        None
    };
    let config = Config::new()?;
    let response = HttpClient::new(&config).validate("/cli", api_key).await?;
    if response.status().is_success() {
        let resp: LoginResp = response
            .json()
            .await
            .context("Failed to send validation request")?;

        let mut config = Config::new()?;
        config.set_user(
            api_key.to_string(),
            resp.org_id.clone(),
            Some(resp.user_email.clone()),
            Some(resp.org_name.clone()),
        );
        config.set_default_org(&resp.org_id);
        config.write()?;

        if let Some(spinner) = spinner {
            spinner.finish_with_message(format!(
                "\nAuthenticated as {} for organization: {}.\nDefault organization set to {}.",
                resp.user_email, resp.org_name, resp.org_name
            ));
        }

        Ok(())
    } else {
        anyhow::bail!("Failed to validate API key. {}", response.status());
    }
}
