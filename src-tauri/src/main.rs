// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commandes;
use std::sync::{Arc, Mutex};

use commandes::{
        get_interfaces, 
        get_status, 
        update_status, 
        start_packet_sending,
        pause_packet_sending
    };

mod tauri_state;
use tauri_state::SimPcapState;

mod errors;
use errors::Error;

fn main() -> Result<(), tauri::Error>{
    // Create the SystemState
    let sim_state = Arc::new(Mutex::new(SimPcapState::default()));
    // Create a new reference to it
    let arced_state = Arc::clone(&sim_state);
    // Pass the reference into a new background thread that increments the power 
    // by 1 once per second, just so we can see that it's doing something
    std::thread::spawn(move || -> Result<(), Error> {
        loop {
            if arced_state.lock()?.sim_status {
                std::thread::sleep(std::time::Duration::from_secs(1));
                let mut state = arced_state.lock()?;
                state.packet_sended = state.packet_sended + 1;
                println!("state: {:?}", state);
            } else {
                std::thread::sleep(std::time::Duration::from_secs(1));
                let state = arced_state.lock()?;
                println!("state: {:?}", state);
                
            }
        }
    });
    
    tauri::Builder::default()
        // Manage our arced SystemState
        .manage(sim_state)
        .invoke_handler(tauri::generate_handler![
            get_interfaces,
            get_status,
            update_status,
            start_packet_sending,
            pause_packet_sending
        ])
        .run(tauri::generate_context!())
        
}
