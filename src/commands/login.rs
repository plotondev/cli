use anyhow::{Context, Result};
use keyring::Entry;
use reqwest;
use serde::Serialize;
use std::io;

#[derive(Serialize)]
struct ApiKeyPayload {
    api_key: String,
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
        let keyring = Entry::new("ploton", "api_key")?;
        keyring
            .set_password(api_key)
            .context("Failed to store API key in keyring")?;

        println!("Ploton auth success.");
        Ok(())
    } else {
        anyhow::bail!(
            "Failed to validate API key. HTTP Error: {}",
            response.status()
        );
    }
}
