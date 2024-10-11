mod commands;
mod events;
mod types;

use commands::{add_server, get_chat, get_servers, greet, list_users, rcon_command, remove_server};
use specta_typescript::Typescript;
use tauri_specta::*;
use types::{AppConfigManager, ArkVersion, ServerConfig, ServerUser};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![
            greet,
            rcon_command,
            list_users,
            add_server,
            get_servers,
            remove_server,
            get_chat
        ])
        .events(tauri_specta::collect_events![
            crate::events::NewServerListEvent
        ])
        .typ::<ServerConfig>()
        .typ::<ServerUser>()
        .typ::<ArkVersion>();

    #[cfg(debug_assertions)]
    builder
        .export(
            Typescript::default()
                .formatter(specta_typescript::formatter::prettier)
                .header("/* eslint-disable */"),
            "../src/lib/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(builder.invoke_handler())
        .setup(|app| {
            // Do this on launch to insure the config file exists
            let _ = AppConfigManager::new(app.handle().clone()).load_or_create();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
