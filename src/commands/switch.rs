use anyhow::Result;
use clap::Parser;
use inquire::{InquireError, Select};

use crate::util::config_file::Config;

/// Switch default organization
#[derive(Parser)]
pub struct Args {}

pub async fn command(_args: Args) -> Result<()> {
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
            config.set_default_org(choice);

            //get user emailt to display as  message
            let user_email = config.get_user_id_by_org_name(choice).unwrap();

            config.write()?;
            println!("Default organization set to {}({})", choice, user_email);
        }
        Err(_) => println!("There was an error, please try again"),
    }
    Ok(())
}
