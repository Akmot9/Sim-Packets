mod commandes;
mod errors;
// Exposé pour l'exemple de vérification `examples/count_check.rs`.
pub mod tauri_state;

use std::sync::{Arc, Mutex};

use commandes::{get_interfaces, pause_packet_sending, start_packet_sending};
use tauri_state::SimPcapState;

pub fn run() -> Result<(), tauri::Error> {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        // Manage our arced SystemState
        .manage(Arc::new(Mutex::new(SimPcapState::default())))
        .invoke_handler(tauri::generate_handler![
            get_interfaces,
            start_packet_sending,
            pause_packet_sending
        ])
        .run(tauri::generate_context!())
}
