use anyhow::{Context, Result};
use keyring::Entry;
pub async fn get_key(org_id: String) -> Result<String> {
    Ok(Entry::new("ploton", &org_id)?.get_password()?)
}

pub async fn set_key(org_id: String, api_key: String) -> Result<()> {
    Entry::new("ploton", &org_id)?
        .set_password(&api_key)
        .context("Failed to store API key in keyring")?;
    Ok(())
}
