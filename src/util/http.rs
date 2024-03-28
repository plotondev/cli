use crate::util::config_file::Config;
use crate::SERVER_URL;
use anyhow::{Context, Result};
use reqwest::{Client, Response};
use serde::Serialize;
pub struct HttpClient<'a> {
    client: Client,
    config: &'a Config,
    base_url: String,
}

impl<'a> HttpClient<'a> {
    pub fn new(config: &'a Config) -> Self {
        HttpClient {
            client: Client::new(),
            config,
            base_url: SERVER_URL.to_string(),
        }
    }

    pub async fn validate(&self, endpoint: &str, api_key: &str) -> Result<Response> {
        let full_url = format!("{}{}", self.base_url, endpoint);
        self.client
            .post(&full_url)
            .header("PLOTON_API_KEY", api_key)
            .send()
            .await
            .context("HTTP POST request failed")
    }

    pub async fn post<T: Serialize>(&self, endpoint: &str, body: &T) -> Result<Response> {
        let full_url = format!("{}{}", self.base_url, endpoint);
        print!("{}", full_url);
        self.client
            .post(&full_url)
            .header(
                "PLOTON_API_KEY",
                self.config.get_default_org().context("No default organization selected. Please run ploton switch to select an organization.")?,
            )
            .json(body)
            .send()
            .await
            .context("HTTP POST request failed")
    }
}
