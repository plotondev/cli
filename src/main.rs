use clap::{Arg, Command};
use tokio;

#[tokio::main]
async fn main() {
    let matches = Command::new("ploton")
        .version("1.0")
        .author("Your Name")
        .about("Pushes code to a server, excluding unwanted files")
        .subcommand(
            Command::new("link")
                .about("Links an app to the current folder")
                .arg(
                    Arg::new("APP_ID")
                        .help("The app ID to link")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(Command::new("push").about("Pushes the current folder's code to the server"))
        .get_matches();

    match matches.subcommand() {
        Some(("link", sub_m)) => {
            let app_id = sub_m.get_one::<String>("APP_ID").unwrap();
            link_app(app_id).await;
        }
        Some(("push", _)) => {
            push_code().await;
        }
        _ => unreachable!("The CLI parser guards against this"),
    }
}

async fn link_app(app_id: &str) {
    println!("Linking app ID: {}", app_id);
}

async fn push_code() {
    println!("Pushing code...");
}
