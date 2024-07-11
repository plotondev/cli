use anyhow::{Context, Result};
use reqwest::{Client, Response};
use serde::Serialize;

use crate::util::config_file::Config;

pub struct HttpClient<'a> {
    client: Client,
    config: &'a Config,
    base_url: String,
}

impl<'a> HttpClient<'a> {
    pub fn new(config: &'a Config) -> Result<Self> {
        let base_url: String = config
            .get_server_url()
            .context("No server URL found. Please run ploton init to set a server URL.")?;
        Ok(HttpClient {
            client: Client::new(),
            config,
            base_url,
        })
    }

    pub async fn validate(&self, api_key: &str) -> Result<Response> {
        let full_url = format!("{}{}", self.base_url, "/auth/verify_auth");
        self.client
            .post(&full_url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .context("HTTP request failed. Kindly try again.")
    }

    pub async fn post<T: Serialize>(&self, endpoint: &str, body: &T) -> Result<Response> {
        let full_url = format!("{}{}", self.base_url, endpoint);

        let default_org = self.config.get_default_org().context(
            "No default organization selected. Please run ploton switch to select an organization.",
        )?;
        let token = self.config.get_user_token_by_org(default_org.as_str()).context("No user found for the default organization. Please run ploton login to re-authenticate.")?;
        self.client
            .post(&full_url)
            .header("PLOTON_API_KEY", token)
            .json(body)
            .send()
            .await
            .context("HTTP request failed. Please try again.")
    }
}
