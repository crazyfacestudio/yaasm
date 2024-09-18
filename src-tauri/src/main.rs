// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// use gamedig::games::asa;
// use rcon_client::{AuthRequest, RCONClient, RCONConfig, RCONError, RCONRequest};
// use std::net::IpAddr;
// use std::str::FromStr;


#[tauri::command]
fn greet(name: &str) -> String {
    println!("I was invoked from JS!");
    // gamedig_call();
    format!("Hello, {}!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let mut ctx = tauri::generate_context!();

    tauri::Builder::default()
    .plugin(tauri_plugin_theme::init(ctx.config_mut()))
        .plugin(tauri_plugin_window_state::Builder::default().build())
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
