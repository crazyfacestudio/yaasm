use anyhow::anyhow;
use gamedig::games::epic::asa;
use rcon::{AsyncStdStream, Connection};
use std::{net::IpAddr, str::FromStr};

#[tauri::command]
#[specta::specta]
pub fn greet(name: &str) -> String {
    println!("I was invoked from JS!");
    gamedig_call();
    format!("Hello, {}!", name)
}

#[tauri::command]
#[specta::specta]
pub async fn rcon_command(_server_id: &str, command: &str) -> Result<String, tauri::Error> {
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
