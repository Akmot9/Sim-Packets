// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commandes;
use std::sync::{Arc, Mutex};

use commandes::{get_interfaces, pause_packet_sending, start_packet_sending};

mod tauri_state;
use tauri_state::SimPcapState;

mod errors;

fn main() -> Result<(), tauri::Error> {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::LogDir {
                file_name: Some("logs".to_string()),
                },
            ))
            .max_file_size(50_000 /* bytes */)
            .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
            .build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        // Manage our arced SystemState
        .manage(Arc::new(Mutex::new(SimPcapState::default())))
        .invoke_handler(tauri::generate_handler![
            get_interfaces,
            start_packet_sending,
            pause_packet_sending
        ])
        .run(tauri::generate_context!())
}
