// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod commands;
mod events;
mod types;

use std::io::Write;
use std::path;

use commands::{greet, rcon_command};
use specta_typescript::Typescript;
use tauri_plugin_fs::FsExt;
use tauri_plugin_fs::OpenOptions;
use tauri_specta::*;
use types::AppConfig;
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
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_deep_link::init())
        // .plugin(tauri_plugin_single_instance::init())
        .plugin(tauri_plugin_theme::init(ctx.config_mut()))
        // .plugin(tauri_plugin_window_state::Builder::default().build())
        // .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let scope = app.fs_scope();
            let fs = app.fs();

            let config_path_str = format!(
                "{}/config.yaml",
                tauri::path::BaseDirectory::AppData.variable()
            );
            let config_path = path::Path::new(&config_path_str);

            scope.allow_file(config_path);

            match fs.read_to_string(config_path) {
                Ok(v) => {
                    let _config: AppConfig = serde_yaml::from_str(&v).unwrap();
                }
                Err(_) => {
                    let config: AppConfig = AppConfig::default();

                    let opts = OpenOptions::new();

                    let mut new_config = fs
                        .open(config_path, opts)
                        .expect("Failed to create config file");

                    new_config
                        .write_all(serde_yaml::to_string(&config).unwrap().as_bytes())
                        .expect("Failed to write config file");
                }
            };

            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        .invoke_handler(tauri::generate_handler![greet, rcon_command])
        .run(ctx)
        .expect("error while running tauri application");
}
