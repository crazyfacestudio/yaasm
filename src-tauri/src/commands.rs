use gamedig::games::epic::asa;
use rcon::{AsyncStdStream, Connection};
use std::{net::IpAddr, str::FromStr};
use tauri::AppHandle;

use crate::types::{AppConfigManager, ServerConfig, ServerUser};

const EMPTY_RCON_RESPONSE: &str = "Server received, But no response";

#[tauri::command]
#[specta::specta]
pub fn greet(name: &str) -> String {
    println!("I was invoked from JS!");
    gamedig_call();
    format!("Hello, {}!", name)
}

#[tauri::command]
#[specta::specta(error = "String")]
pub async fn add_server(app_handle: AppHandle, server: ServerConfig) -> Result<bool, String> {
    let server_config = server.clone().add_id();
    let config_manager = AppConfigManager::new(app_handle.clone());
    let mut config = config_manager.load_or_create().unwrap();

    if config.check_duplicate_server(&server_config) {
        println!("Server already exists");
        return Err("Server already exists".to_string());
    }

    config.servers.push(server_config);

    match config_manager.save(&config) {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
#[specta::specta(error = "String")]
pub async fn remove_server(app_handle: AppHandle, server_id: String) -> Result<bool, String> {
    let config_manager = AppConfigManager::new(app_handle.clone());
    let mut config = config_manager.load_or_create().unwrap();

    let mut new_servers = Vec::<ServerConfig>::new();
    for server in config.servers.iter() {
        if server_id != server.id().clone().unwrap() {
            new_servers.push(server.clone());
        }
    }

    config.servers = new_servers;

    match config_manager.save(&config) {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
#[specta::specta(error = "String")]
pub async fn get_servers(app_handle: AppHandle) -> Result<Vec<ServerConfig>, String> {
    let config_manager = AppConfigManager::new(app_handle.clone());
    let config = config_manager.load_or_create().unwrap();

    Ok(config.servers)
}

#[tauri::command]
#[specta::specta(error = "String")]
pub async fn list_users(app_handle: AppHandle, server_id: &str) -> Result<Vec<ServerUser>, String> {
    let config_manager = AppConfigManager::new(app_handle.clone());
    let config = config_manager.load_or_create().unwrap();
    let server = config.get_server_by_id(server_id).unwrap();

    let command = "listplayers";
    let address = format!("{}:{}", server.host(), server.rcon_port());
    let conn = <Connection<AsyncStdStream>>::builder()
        .enable_factorio_quirks(true)
        .connect(address.clone(), server.rcon_password())
        .await;

    match conn {
        Ok(mut con) => {
            println!("connected to server: {}", address.clone());
            println!("Sending command: {}", command);
            match con.cmd(command).await {
                Ok(reply) => {
                    let mut users = Vec::<ServerUser>::new();
                    for part in reply.trim().lines() {
                        if reply.starts_with(&EMPTY_RCON_RESPONSE.to_string()) {
                            return Ok(vec![]);
                        }
                        match ServerUser::from_rcon_user(part.to_string()) {
                            Ok(user) => users.push(user),
                            Err(e) => {
                                println!("Failed to parse user: {}", e);
                            }
                        }
                    }

                    Ok(users)
                }
                Err(e) => {
                    println!("Failed to send listplayers command: {}", e);
                    Err(format!("Failed to send listplayers command: {}", e))
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to server: {}", e);
            Err(format!("Failed to connect to server: {}", e))
        }
    }
}

#[tauri::command]
#[specta::specta(error = "String")]
pub async fn get_chat(app_handle: AppHandle, server_id: &str) -> Result<Vec<String>, String> {
    let config_manager = AppConfigManager::new(app_handle.clone());
    let config = config_manager.load_or_create().unwrap();
    let server = config.get_server_by_id(server_id).unwrap();

    let command = "getchat";
    let address = format!("{}:{}", server.host(), server.rcon_port());
    let conn = <Connection<AsyncStdStream>>::builder()
        .enable_factorio_quirks(true)
        .connect(address.clone(), server.rcon_password())
        .await;

    match conn {
        Ok(mut con) => {
            println!("connected to server: {}", address.clone());
            println!("Sending command: {}", command);
            match con.cmd(command).await {
                Ok(reply) => {
                    if reply.starts_with(&EMPTY_RCON_RESPONSE.to_string()) {
                        return Ok(vec!["No chat messages".to_string()]);
                    }
                    Ok(reply.trim().lines().map(|s| s.to_string()).collect())
                }
                Err(e) => {
                    println!("Failed to send getchat command: {}", e);
                    Err(format!("Failed to send getchat command: {}", e))
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to server: {}", e);
            Err(format!("Failed to connect to server: {}", e))
        }
    }
}

#[tauri::command]
#[specta::specta(error = "String")]
pub async fn rcon_command(_server_id: &str, command: &str) -> Result<String, String> {
    println!("I was invoked from JS!");

    let address = "31.214.196.6:11600";
    let conn = <Connection<AsyncStdStream>>::builder()
        .enable_factorio_quirks(true)
        .connect(address, "mommy12345678")
        .await;

    match conn {
        Ok(mut con) => {
            println!("connected to server: {}", address);
            println!("Sending command: {}", command);
            match con.cmd(command).await {
                Ok(reply) => Ok(reply),
                Err(e) => Err(format!("Failed to send command: {}", e)),
            }
        }
        Err(e) => Err(format!("Failed to connect to server: {}", e)),
    }
}

fn gamedig_call() {
    let server_ip = "31.214.196.6";
    let server_port = Some(5600);

    let ip: IpAddr = IpAddr::from_str(server_ip).expect("Invalid IP address");

    match asa::query(&ip, server_port) {
        Ok(response) => {
            println!("Server: {:#?}", response);
            println!("Server name: {}", response.name);
            println!(
                "Players: {}/{}",
                response.players_online, response.players_maxmimum
            );
            for player in response.players {
                println!("Player: {}", player.name);
            }
        }
        Err(e) => eprintln!("Failed to query server: {}", e),
    }
}
