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
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlotonUser {
    pub token: String,
    pub email: Option<String>,
    pub org_name: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct PlotonConfig {
    pub user: HashMap<String, PlotonUser>, //org_id -> user
    pub default_org: Option<String>,
    pub projects: HashMap<String, LinkedProject>, //project_id -> project
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
                default_org: None,
                projects: HashMap::new(),
                user: HashMap::new(),
            },
        })
    }

    pub fn get_current_directory(&self) -> Result<String> {
        let current_dir = std::env::current_dir()?;
        let path = current_dir
            .to_str()
            .context("Unable to get current working directory")?;
        Ok(path.to_owned())
    }
    pub fn link_project(&mut self, project_id: String, name: Option<String>) -> Result<()> {
        let project_path = self.get_current_directory()?;
        self.config
            .projects
            .insert(project_id.clone(), LinkedProject { project_path, name });
        Ok(())
    }

    pub fn set_user(
        &mut self,
        token: String,
        org_id: String,
        email: Option<String>,
        org_name: Option<String>,
    ) {
        self.config.user.insert(
            org_id.clone(),
            PlotonUser {
                token,
                email,
                org_name,
            },
        );
    }

    fn get_org_id_by_org_name(&self, org_name: &str) -> Option<String> {
        self.config
            .user
            .iter()
            .find(|(_, u)| u.org_name.as_deref() == Some(org_name))
            .map(|(id, _)| id.clone())
    }
    pub fn get_user_id_by_org_name(&self, org_name: &str) -> Option<String> {
        self.config
            .user
            .get(&self.get_org_id_by_org_name(org_name).unwrap())
            .map(|u| u.email.clone())
            .flatten()
    }
    pub fn set_default_org(&mut self, org_name: &str) {
        self.config.default_org = self.get_org_id_by_org_name(&org_name);
    }

    pub fn get_user_token_by_org(&self, org_id: &str) -> Option<String> {
        self.config.user.get(org_id).map(|u| u.token.clone())
    }
    pub fn get_default_user_token(&self) -> Option<String> {
        self.get_default_org()
            .and_then(|org_id| self.get_user_token_by_org(&org_id))
    }
    pub fn get_default_org(&self) -> Option<String> {
        self.config.default_org.clone()
    }
    pub fn get_all_users(&self) -> Vec<&PlotonUser> {
        self.config.user.values().collect()
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
