use regex::Regex;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{
    fs::{self, OpenOptions},
    io::Write,
};
use tauri::{AppHandle, Manager};
use tauri_plugin_fs::FsExt;
use thiserror::Error;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
#[serde(tag = "type", content = "data")]
pub enum ArkVersion {
    ASA,
    ASE,
}

#[derive(Error, Debug, Serialize, Type)]
#[serde(tag = "type", content = "data")]
pub enum AppConfigError {
    // On the frontend this variant will be "AnotherError" with string data.
    #[error("Failed to resolve appdata path: {0}")]
    ResolveAppDataPath(String),
    // On the frontend this variant will be "AnotherError" with string data.
    #[error("Failed to resolve config file path: {0}")]
    ResolveConfigPath(String),
    // On the frontend this variant will be "AnotherError" with string data.
    #[error("Save failed: {0}")]
    SaveFailed(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct ServerConfig {
    #[allow(unused)]
    id: Option<String>,
    #[allow(unused)]
    server_type: ArkVersion,
    #[allow(unused)]
    name: String,
    #[allow(unused)]
    host: String,
    #[allow(unused)]
    query_port: u16,
    #[allow(unused)]
    rcon_port: u16,
    #[allow(unused)]
    rcon_password: String,
}

impl ServerConfig {
    pub fn add_id(&self) -> Self {
        let id = Uuid::new_v4().to_string();
        Self {
            id: Some(id),
            ..self.clone()
        }
    }

    #[allow(unused)]
    pub fn id(&self) -> &Option<String> {
        &self.id
    }

    #[allow(unused)]
    pub fn name(&self) -> &String {
        &self.name
    }
    #[allow(unused)]
    pub fn host(&self) -> &String {
        &self.host
    }
    #[allow(unused)]
    pub fn query_port(&self) -> &u16 {
        &self.query_port
    }
    #[allow(unused)]
    pub fn rcon_port(&self) -> &u16 {
        &self.rcon_port
    }
    #[allow(unused)]
    pub fn rcon_password(&self) -> &String {
        &self.rcon_password
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NitradoKey {
    pub name: String,
    pub key: String,
}

pub struct AppConfigManager {
    app_handle: AppHandle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub servers: Vec<ServerConfig>,
    pub nitrado_keys: Vec<NitradoKey>,
}

impl AppConfig {
    pub fn check_duplicate_server(&self, server: &ServerConfig) -> bool {
        for server_config in self.servers.iter() {
            if server_config.host() == server.host()
                && server_config.rcon_port() == server.rcon_port()
            {
                return true;
            }
        }
        false
    }

    pub fn get_server_by_id(&self, id: &str) -> Option<&ServerConfig> {
        for server_config in self.servers.iter() {
            if server_config.id().clone().unwrap().as_str() == id {
                return Some(server_config);
            }
        }
        None
    }
}

impl AppConfigManager {
    #[allow(unused)]
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn load_or_create(&self) -> Result<AppConfig, AppConfigError> {
        let config: AppConfig;
        let scope = self.app_handle.fs_scope();
        let fs = self.app_handle.fs();
        let app_data = match self
            .app_handle
            .path()
            .resolve("", tauri::path::BaseDirectory::AppData)
        {
            Ok(v) => v,
            Err(e) => {
                return Err(AppConfigError::ResolveAppDataPath(e.to_string()));
            }
        };

        fs::create_dir_all(app_data.clone()).unwrap();

        let path = match self
            .app_handle
            .path()
            .resolve("config.yaml", tauri::path::BaseDirectory::AppData)
        {
            Ok(v) => v,
            Err(e) => {
                return Err(AppConfigError::ResolveConfigPath(e.to_string()));
            }
        };

        scope.allow_file(path.clone());
        scope.allow_directory(app_data.clone(), true);

        match fs.read_to_string(path.clone()) {
            Ok(v) => {
                println!("Loading config...");
                config = serde_yaml::from_str(&v).unwrap();
                println!("Config loaded...");
            }
            Err(_) => {
                println!("Creating new config...");
                config = AppConfig::default();

                let mut new_config = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(path.clone())
                    .expect("Failed to create config file");

                new_config
                    .write_all(serde_yaml::to_string(&config).unwrap().as_bytes())
                    .expect("Failed to write config file");
            }
        };

        Ok(config)
    }

    pub fn save(&self, config: &AppConfig) -> Result<bool, AppConfigError> {
        let path = match self
            .app_handle
            .path()
            .resolve("config.yaml", tauri::path::BaseDirectory::AppData)
        {
            Ok(v) => v,
            Err(e) => {
                return Err(AppConfigError::ResolveConfigPath(e.to_string()));
            }
        };

        let mut new_config = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path.clone())
            .expect("Failed to create config file");

        println!("Saving..");

        match new_config.write_all(serde_yaml::to_string(&config).unwrap().as_bytes()) {
            Ok(_) => Ok(true),
            Err(e) => Err(AppConfigError::SaveFailed(e.to_string())),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        // Return a default instance of AppConfig
        AppConfig {
            servers: Vec::new(),
            nitrado_keys: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct ServerUser {
    name: String,
    id: String,
}

impl ServerUser {
    pub fn from_rcon_user(user: String) -> Result<Self, String> {
        let re = Regex::new(r"^(\d+\.) ([\w ]+), (.*)$").unwrap();
        let captures = re.captures(&user);

        match captures {
            Some(captures) => {
                let name = captures.get(2).unwrap().as_str().to_string();
                let id = captures.get(3).unwrap().as_str().to_string();
                println!("User: {}", name);
                Ok(Self { name, id })
            }
            None => {
                println!("Failed to parse user: {}", user);
                Err(format!("Failed to parse rcon user: {}", user))
            }
        }
    }
}
