use crate::types::ServerConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, specta::Type, tauri_specta::Event)]
pub struct NewServerListEvent(Vec<ServerConfig>);
