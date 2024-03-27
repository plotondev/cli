use anyhow::Result;
use clap::Parser;

/// Push the current app to the server
#[derive(Parser)]
pub struct Args {}
//1) bundle the code
//2) send the code to the server
pub async fn command(_args: Args, json: bool) -> Result<()> {
    println!("Pushing code...");
    Ok(())
}
