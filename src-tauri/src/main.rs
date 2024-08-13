// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commandes;
use commandes::{get_interfaces,get_status, update_status};

mod tauri_state;
use tauri_state::SimPcapState;

mod errors;

fn main() {
    tauri::Builder::default()
    .manage(SimPcapState::default())
        .invoke_handler(tauri::generate_handler![
            get_interfaces,
            get_status,
            update_status
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
