// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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

use tauri::Manager;
use tauri::GlobalShortcutManager;

fn main() -> Result<(), tauri::Error>{
    tauri::Builder::default()
        // Manage our arced SystemState
        .manage( Arc::new(Mutex::new(SimPcapState::default())))
        .setup(|app| {
            let app_handle = app.handle(); // Get the AppHandle
            let window = app_handle.get_window("main").unwrap();
            
            // Open DevTools
            window.open_devtools();
            
            // Register a global shortcut
            let mut shortcuts = app_handle.global_shortcut_manager();
            shortcuts.register("CmdOrCtrl+Shift+I", move || {
              window.open_devtools();
            }).unwrap();
      
            Ok(())
          })
        .invoke_handler(tauri::generate_handler![
            get_interfaces,
            start_packet_sending,
            pause_packet_sending
        ])
        .run(tauri::generate_context!())
        
}
