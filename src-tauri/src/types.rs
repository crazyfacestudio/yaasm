use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
pub struct ServerConfig {
    #[allow(unused)]
    name: String,
    #[allow(unused)]
    host: String,
    #[allow(unused)]
    query_port: u16,
    #[allow(unused)]
    rcon_port: u16,
}

impl ServerConfig {
    #[allow(unused)]
    pub fn new(name: String, host: String, query_port: u16, rcon_port: u16) -> Self {
        Self {
            name,
            host,
            query_port,
            rcon_port,
        }
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NitradoKey {
    pub name: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    servers: Vec<ServerConfig>,
    nitrado_keys: Vec<NitradoKey>,
}

impl AppConfig {
    #[allow(unused)]
    pub fn new(servers: Vec<ServerConfig>, nitrado_keys: Vec<NitradoKey>) -> Self {
        Self {
            servers,
            nitrado_keys,
        }
    }
    #[allow(unused)]
    pub fn servers(&self) -> &Vec<ServerConfig> {
        &self.servers
    }
    #[allow(unused)]
    pub fn nitrado_keys(&self) -> &Vec<NitradoKey> {
        &self.nitrado_keys
    }
}
