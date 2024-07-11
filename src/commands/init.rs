use anyhow::Result;
use clap::Parser;

use crate::util::config_file::Config;

/// Init Ploton cli
#[derive(Parser)]
pub struct Args {
    server: String,
}

pub async fn command(_args: Args) -> Result<()> {
    let mut config = Config::new()?;
    config.set_server(_args.server.clone())?;
    config.write()?;
    println!("Default server set to {}", _args.server);
    Ok(())
}
