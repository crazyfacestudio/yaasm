// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use std::time::Duration;
// use rercon::{Connection, Settings};
use rcon::{AsyncStdStream, Connection};

#[tauri::command]
async fn greet(name: &str) -> Result<String, tauri::Error> {
    println!("I was invoked from JS!");

    let address = "31.214.196.6:11600";
    let conn = <Connection<AsyncStdStream>>::builder()
        // .enable_minecraft_quirks(true)
        .connect(address, "mommy12345678")
        .await;

    match conn {
        Ok(mut con) => {
            println!("connected to server: {}", address);
            println!("Sending command: cheat listplayers");
            match con.cmd("listplayers").await {
                Ok(reply) => {
                    println!("Reply from server: {}", reply);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }

    // let settings = Settings {
    //     connect_timeout: Duration::from_millis(1000),
    //     auth_delay: Some(Duration::from_millis(1000)),
    // };

    // let connection = Connection::open("31.214.196.6:11600", "mommy12345678", settings).await;

    // match connection {
    //     Ok(mut con) => {
    //         println!("connected to server: 31.214.196.6:11600");
    //         println!("Sending command: cheat listplayers");
    //         match con.exec("\ncheat listplayers\n").await {
    //             Ok(reply) => {
    //                 println!("Reply from server: {}", reply);
    //             }
    //             Err(e) => {
    //                 println!("Error: {:?}", e);
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         println!("Error: {:?}", e);
    //     }
    // }

    Ok(format!("Hello, {}!", name))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let mut ctx = tauri::generate_context!();

    tauri::Builder::default()
        // .plugin(tauri_plugin_single_instance::init())
        .plugin(tauri_plugin_theme::init(ctx.config_mut()))
        // .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![greet])
        .run(ctx)
        .expect("error while running tauri application");
}

// fn testing() -> Result<(), RCONError> {
//     println!("Running RCON test");
//     // Create new RCON client
//     let mut client = RCONClient::new(RCONConfig {
//         url: "207.244.127.182:11480".to_string(),
//         // Optional
//         read_timeout: Some(13),
//         write_timeout: Some(37),
//     })?;

//     // Auth request to RCON server (SERVERDATA_AUTH)
//     let auth_result = client.auth(AuthRequest::new("mommy12345678".to_string()))?;
//     assert!(auth_result.is_success());

//     // Execute command request to RCON server (SERVERDATA_EXECCOMMAND)
//     let list_players = client.execute(RCONRequest::new("ListPlayers".to_string()))?;
//     println!("Result: {}", list_players.body);
//     // assert_eq!(version.body, "Seed: [3257840388504953787]");

//     Ok(())
// }

// fn gamedig_call() {
//     let server_ip = "207.244.127.182";
//     let server_port = Some(5480);

//     let ip: IpAddr = IpAddr::from_str(server_ip).expect("Invalid IP address");

//     match asa::query(&ip, server_port) {
//         Ok(response) => {
//             println!("Server: {:?}", response);
//             println!("Server name: {}", response.name);
//             println!(
//                 "Players: {}/{}",
//                 response.players_online, response.players_maxmimum
//             );
//             for player in response.players {
//                 println!("Player: {}", player.name);
//             }
//         }
//         Err(e) => eprintln!("Failed to query server: {}", e),
//     }
// }
