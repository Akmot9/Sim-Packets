mod errors; // Declare the module `errors`, presumably containing custom error handling logic
mod commandes; // Declare the module `commandes`, which likely includes commands or functions related to network packet handling
use std::sync::{Arc, Mutex}; // Import `Arc` (atomic reference counting) and `Mutex` (mutual exclusion) from the standard library for safe concurrent access

mod tauri_state; // Declare the module `tauri_state`, which likely contains shared state management logic
use tauri_state::SimPcapState; // Use `SimPcapState` struct from the `tauri_state` module, which manages the state for the packet capture simulation

use commandes::{
    resume_packet_sending, 
    get_interfaces, 
    pause_packet_sending, 
    start_packet_sending,
    update_simulation_state,
}; // Import specific functions from the `commandes` module

#[cfg_attr(mobile, tauri::mobile_entry_point)] // Macro to define the entry point for mobile platforms if the `mobile` config attribute is set
pub fn run() -> Result<(), tauri::Error> {
    // Entry point of the Tauri application
    tauri::Builder::default()
    
        // Add the Tauri logging plugin to log messages from the application
        .plugin(tauri_plugin_log::Builder::new()
            .clear_targets()
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::Stdout,
            ))
            .build())
        // Add the Tauri process plugin to manage the system processes
        .plugin(tauri_plugin_process::init())
        // Add the Tauri dialog plugin to handle dialogs (e.g., file dialogs or alert dialogs)
        .plugin(tauri_plugin_dialog::init())
        // Add the Tauri shell plugin to handle shell commands (e.g., launching external processes)
        .plugin(tauri_plugin_shell::init())

        // Manage application state by initializing a shared `SimPcapState` inside an `Arc<Mutex<>>` 
        // to allow safe mutable access across threads
        .manage(Arc::new(Mutex::new(SimPcapState::default())))

        // Define the commands that can be invoked from the frontend (JavaScript)
        .invoke_handler(tauri::generate_handler![
            get_interfaces,        // Command to get available network interfaces
            pause_packet_sending,  // Command to pause packet sending
            start_packet_sending,   // Command to start packet sending
            resume_packet_sending,
            update_simulation_state
        ])

        // Start the Tauri runtime with the generated application context
        .run(tauri::generate_context!())
}
