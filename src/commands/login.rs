use std::{io, time::Duration};

use anyhow::{Context, Result};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use is_terminal::IsTerminal;
use serde::Deserialize;

use crate::{
    TICK_STRING,
    util::{config_file::Config, http::HttpClient},
};

/// Ploton authentication command
#[derive(Parser)]
pub struct Args {}

#[derive(Deserialize, Debug)]
struct LoginResp {
    role: String,
    t_id: String,    //tenant id
    r_id: String,    //resource id
    user_id: String, //userid
}

pub async fn command(_args: Args) -> Result<()> {
    let config = Config::new()?;

    println!("Please visit the following URL to obtain your API key:");
    println!(
        "{}/cli/keys",
        config
            .get_server_url()
            .context("No server URL found. Please run ploton init to set a server URL.")?
    );
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

    let response = HttpClient::new(&config)?.validate(api_key).await?;

    if response.status().is_success() {
        let resp: LoginResp = response
            .json()
            .await
            .context("Failed to send validation request")?;

        println!("{:?}", resp);

        let mut config = Config::new()?;

        println!("{:?}", config);
        config.set_user(
            api_key.to_string(),
            resp.t_id.clone(),
            resp.user_id.clone(),
            resp.t_id.clone(),
        );
        config.set_default_org(&resp.t_id);
        config.write()?;

        if let Some(spinner) = spinner {
            spinner.finish_with_message(format!(
                "\nAuthenticated for organization: {}.\nDefault organization set to {}.",
                resp.t_id, resp.t_id
            ));
        }

        Ok(())
    } else {
        anyhow::bail!("Failed to validate API key. {}", response.status());
    }
}
