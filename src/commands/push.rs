use anyhow::Result;

use crate::util::api_key::get_key;
pub async fn execute() -> Result<()> {
    let password = get_key(String::from("api_key")).await?;

    println!("My password is '{}'", password);
    println!("Pushing code...");
    Ok(())
}
