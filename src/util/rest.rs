use anyhow::{Context, Result};
use postgrest::Postgrest;

use super::config_file::Config;

pub struct PGRest<'a> {
    client: Postgrest,
    config: &'a Config,
}
impl<'a> PGRest<'a> {
    pub fn new(config: &'a Config) -> Result<Self> {
        let default_org = config.get_default_org().context(
            "No default organization selected. Please run ploton switch to select an organization.",
        )?;
        let token = config.get_user_token_by_org(default_org.as_str()).context("No user found for the default organization. Please run ploton login to re-authenticate.")?;
        let url: String = config.get_server_url().context("No server URL found. Please run ploton init to set a server URL.")?;

        Ok(PGRest {
            client: Postgrest::new(url)
                .insert_header("Authorization", format!("Bearer {}", token)),
            config,
        })
    }
}
