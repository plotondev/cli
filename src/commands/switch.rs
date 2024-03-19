use anyhow::Result;
use clap::Parser;
use inquire::{InquireError, Select};

use crate::util::config_file::Config;

/// Push the current app to the server
#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: Args, json: bool) -> Result<()> {
    let config = Config::new()?;
    let users = config.get_all_users();

    let options: Vec<&str> = users
        .iter()
        .map(|u| u.org_name.as_ref().unwrap().as_str())
        .collect::<Vec<&str>>();

    let ans: Result<&str, InquireError> =
        Select::new("Switch default organization", options).prompt();

    match ans {
        Ok(choice) => {
            let mut config = Config::new()?;
            config.set_default_org(choice.to_string());
            config.write()?;
            println!("Default organization set to {}", choice);
        }
        Err(_) => println!("There was an error, please try again"),
    }
    Ok(())
}
