// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use gamedig::games::epic::asa;
use std::{net::IpAddr, str::FromStr};

use anyhow::anyhow;
use rcon::{AsyncStdStream, Connection};

#[tauri::command]
async fn greet(name: &str) -> Result<String, tauri::Error> {
    println!("I was invoked from JS!");
    gamedig_call();
    Ok(format!("Hello, {}!", name))
}

#[tauri::command]
async fn rcon_command(_server_id: &str, command: &str) -> Result<String, tauri::Error> {
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
                Err(e) => Err(anyhow!("Command Error: {:?}", e).into()),
            }
        }
        Err(e) => Err(anyhow!("Connection Error: {:?}", e).into()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let mut ctx = tauri::generate_context!();

    tauri::Builder::default()
        // .plugin(tauri_plugin_single_instance::init())
        .plugin(tauri_plugin_theme::init(ctx.config_mut()))
        // .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![greet, rcon_command])
        .run(ctx)
        .expect("error while running tauri application");
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
