use anyhow::Result;
use clap::Parser;

use crate::util::local_file::LocalFile;

/// Push the current app to the server
#[derive(Parser)]
pub struct Args {}
//1) bundle the code
//2) send the code to the server
pub async fn command(_args: Args, json: bool) -> Result<()> {
    let mut local_file = LocalFile::read()?;
    println!("Pushing code...");
    print!("local file {:?}", local_file);
    Ok(())
}
