// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commandes;
use std::sync::{Arc, Mutex};

use commandes::{
        get_interfaces,
        start_packet_sending,
        pause_packet_sending
    };

mod tauri_state;
use tauri_state::SimPcapState;

mod errors;

fn main() -> Result<(), tauri::Error>{
    tauri::Builder::default()
        // Manage our arced SystemState
        .manage( Arc::new(Mutex::new(SimPcapState::default())))
        .invoke_handler(tauri::generate_handler![
            get_interfaces,
            start_packet_sending,
            pause_packet_sending
        ])
        .run(tauri::generate_context!())
        
}
