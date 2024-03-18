use anyhow::{Context, Result};
use dirs;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    fs::{self, create_dir_all, File},
    io::Read,
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkedProject {
    pub project_path: String,
    pub name: Option<String>,
    pub project: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlotonConfig {
    pub org_id: Option<String>,
    pub projects: HashMap<String, LinkedProject>,
}

pub struct Config {
    root_config_path: PathBuf,
    config: PlotonConfig,
}

impl Config {
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir().context("Home directory not found")?;
        let root_config_path = std::path::Path::new(&home_dir).join(".ploton/config.json");

        if let Ok(mut file) = File::open(&root_config_path) {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let config: PlotonConfig = serde_json::from_str(&contents)?;
            return Ok(Self {
                root_config_path,
                config,
            });
        }

        Ok(Self {
            root_config_path,
            config: PlotonConfig {
                org_id: None,
                projects: HashMap::new(),
            },
        })
    }

    pub fn get_org_id(&self) -> Option<String> {
        self.config.org_id.clone()
    }

    pub fn set_org_id(&mut self, org_id: String) {
        self.config.org_id = Some(org_id);
    }

    pub fn write(&self) -> Result<()> {
        let config_dir = self
            .root_config_path
            .parent()
            .context("Failed to get parent directory")?;

        // Ensure directory exists
        create_dir_all(config_dir)?;

        // Use temporary file to achieve atomic write:
        //  1. Open file ~/ploton/config.tmp
        //  2. Serialize config to temporary file
        //  3. Rename temporary file to ~/ploton/config.json (atomic operation)
        let tmp_file_path = self.root_config_path.with_extension("tmp");
        let tmp_file = File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&tmp_file_path)?;
        serde_json::to_writer_pretty(&tmp_file, &self.config)?;
        tmp_file.sync_all()?;

        // Rename file to final destination to achieve atomic write
        fs::rename(tmp_file_path.as_path(), &self.root_config_path)?;

        Ok(())
    }
}
