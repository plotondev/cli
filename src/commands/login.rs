use keyring::{Entry, Error};
use std::io;

pub async fn execute() -> Result<(), Error> {
    println!("Please visit the following URL to obtain your API key:");
    println!("https://auth.ploton.dev/org/api_keys/");
    println!("Enter your API key:");

    let mut api_key = String::new();
    io::stdin()
        .read_line(&mut api_key)
        .expect("Failed to read line");
    let api_key = api_key.trim(); // Trim newline and whitespaces

    // Using 'ploton' as the service identifier and 'api_key' as the account name.
    let keyring = Entry::new("ploton", "api_key")?;
    match keyring.set_password(api_key) {
        Ok(_) => {
            println!("API key stored successfully.");
            Ok(()) // Explicitly return Ok(())
        }
        Err(e) => {
            println!("Failed to store API key: {}", e);
            Err(e) // Propagate the error
        }
    }
}
