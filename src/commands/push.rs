use anyhow::Result;
use clap::Parser;

/// Push the current app to the server
#[derive(Parser)]
pub struct Args {}
use crate::util::api_key::get_key;

pub async fn command(_args: Args, json: bool) -> Result<()> {
    let password = get_key(String::from("api_key")).await?;

    println!("My password is '{}'", password);
    println!("Pushing code...");
    Ok(())
}
