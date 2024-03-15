use crate::SERVER_URL;
use anyhow::{Context, Result};
use reqwest::{Client, Response};

pub struct HttpClient {
    client: Client,
    base_url: String,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            client: Client::new(),
            base_url: SERVER_URL.to_string(),
        }
    }

    pub async fn post(&self, endpoint: &str, api_key: &str) -> Result<Response> {
        let full_url = format!("{}{}", self.base_url, endpoint);
        self.client
            .post(&full_url)
            .header("PLOTON_API_KEY", api_key)
            .send()
            .await
            .context("HTTP POST request failed")
    }
}
