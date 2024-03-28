use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::*;
use tokio;

mod commands;
mod macros;
mod util;

static LOGIN_URL: &str = env!("LOGIN_URL");
static SERVER_URL: &str = env!("SERVER_URL");
static TICK_STRING: &str = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ";
/// Interact with Ploton via CLI
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    command: Commands,

    /// Output in JSON format
    #[clap(global = true, long)]
    json: bool,
}

commands_enum!(link, login, push, switch, new);

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Args::parse();

    match Commands::exec(cli).await {
        Ok(_) => {}
        Err(e) => {
            // If the user cancels the operation, we want to exit successfully
            // This can happen if Ctrl+C is pressed during a prompt
            if e.root_cause().to_string() == inquire::InquireError::OperationInterrupted.to_string()
            {
                return Ok(());
            }

            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
