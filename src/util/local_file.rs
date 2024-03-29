use std::{fmt, fs::File, io::Read};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct LocalUserInput {
    pub name: String,
    pub description: String,
    pub default: Option<String>,
    pub required: bool,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct LocalFile {
    pub id: String,
    pub name: String,
    pub version: String,
    pub logo: String,
    pub description: String,
    pub platform: String,
    pub main: String,
    pub readme: String,
    pub user_inputs: Vec<LocalUserInput>,
}

#[derive(Debug)]
pub enum LocalFileError {
    FileNotFound,
    AlreadyExists,
}

impl fmt::Display for LocalFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocalFileError::FileNotFound => {
                write!(f, "Cannot find ploton.yaml file in the current directory.")
            }
            LocalFileError::AlreadyExists => {
                write!(f, "ploton.yaml already exists in the current directory.")
            }
        }
    }
}

impl std::error::Error for LocalFileError {}

impl LocalFile {
    //used whie reading the local ploton yaml file
    pub fn read() -> Result<Self> {
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;
        let config_path = current_dir.join("ploton.yaml");

        if let Ok(mut file) = File::open(&config_path) {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let config: LocalFile = serde_yaml::from_str(&contents)?;
            return Ok(config);
        }
        Err(LocalFileError::FileNotFound)?
    }

    //when creating a new local ploton yaml file
    pub fn new() -> Result<Self> {
        if Self::file_exists() {
            Err(LocalFileError::AlreadyExists)?
        }
        Ok(Self {
            ..Default::default()
        })
    }
    pub fn load(&mut self, local_file: LocalFile) {
        *self = local_file;
    }

    fn file_exists() -> bool {
        let current_dir = std::env::current_dir().ok();
        let config_path = current_dir.map(|dir| dir.join("ploton.yaml"));
        config_path.map(|path| path.exists()).unwrap_or(false)
    }

    pub fn write(&self) -> Result<()> {
        let current_dir = std::env::current_dir().context("Failed to get current directory")?;
        let config_path = current_dir.join("ploton.yaml");

        let local_file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&config_path)?;
        serde_yaml::to_writer(&local_file, &self)?;
        local_file.sync_all()?;

        Ok(())
    }
}
