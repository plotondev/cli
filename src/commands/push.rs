use keyring::{Entry, Result};
pub async fn execute() -> Result<()> {
    let entry = Entry::new("ploton", "api_key")?;
    let password = entry.get_password()?;
    println!("My password is '{}'", password);
    println!("Pushing code...");
    Ok(())
}
