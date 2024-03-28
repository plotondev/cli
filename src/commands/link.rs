use anyhow::Result;
use clap::Parser;

use crate::util::config_file::Config;

/// Link an app to the current folder
#[derive(Parser)]
pub struct Args {
    app_id: Option<String>,
}

pub async fn command(_args: Args, json: bool) -> Result<()> {
    let mut config = Config::new()?;
    if let Some(app_id) = _args.app_id {
        //check if project exists
        println!("Linking app ID: {}", app_id);
        config.link_project(app_id, None)?;
        config.write()?;
        return Ok(());
    } else {
        println!("No app ID provided. Read config file to find app ID.");
    }
    // Implement the linking logic here
    Ok(())
}
