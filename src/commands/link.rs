use anyhow::Result;
use clap::Parser;

use crate::util::config_file::Config;

/// Show information about the current project
#[derive(Parser)]
pub struct Args {
    app_id: String,
}

pub async fn command(_args: Args, json: bool) -> Result<()> {
    let mut config = Config::new()?;

    println!("Linking app ID: {} {}", _args.app_id, json);

    config.link_project(_args.app_id, None)?;
    config.write()?;
    // Implement the linking logic here
    Ok(())
}
