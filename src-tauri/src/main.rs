// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod events;
mod types;

use commands::{greet, rcon_command};
use serde_json::json;
use specta_typescript::Typescript;
use tauri_plugin_store::StoreExt;
use tauri_specta::*;
use types::ServerConfig;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let builder = Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![greet,])
        .events(tauri_specta::collect_events![
            crate::events::NewServerListEvent
        ])
        .typ::<ServerConfig>();

    #[cfg(debug_assertions)]
    builder
        .export(
            Typescript::default()
                .formatter(specta_typescript::formatter::prettier)
                .header("/* eslint-disable */"),
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    let mut ctx = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        // .plugin(tauri_plugin_single_instance::init())
        .plugin(tauri_plugin_theme::init(ctx.config_mut()))
        // .plugin(tauri_plugin_window_state::Builder::default().build())
        // .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let store = app.handle().store_builder("store.bin").build();

            let value = store.get("servers");
            match value {
                Some(v) => {
                    let _servers: Vec<ServerConfig> =
                        serde_json::from_value::<Vec<ServerConfig>>(v).unwrap();
                    // app.manage(servers);
                }
                None => {
                    let servers: Vec<ServerConfig> = vec![];
                    store.set("servers", json!(servers));
                    store.save()?;
                    // app.manage(servers);
                }
            }

            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        .invoke_handler(tauri::generate_handler![greet, rcon_command])
        .run(ctx)
        .expect("error while running tauri application");
}
