use anyhow::Result;
use clap::Parser;

/// Show information about the current project
#[derive(Parser)]
pub struct Args {
    app_id: String,
}

pub async fn command(_args: Args, json: bool) -> Result<()> {
    println!("Linking app ID: {} {}", _args.app_id, json);
    // Implement the linking logic here
    Ok(())
}
