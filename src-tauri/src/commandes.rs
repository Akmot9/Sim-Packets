use crate::tauri_state::SimPcapState;
use pnet::datalink;
use std::sync::{Arc, Mutex};
use tauri::{command, State};
use crate::errors::Error;

#[command(async)]
pub fn get_interfaces() -> Result<Vec<String>, Error> {
    // Attempt to retrieve the list of all network interfaces via the pnet datalink module.
    let interfaces = datalink::interfaces();

    // Check if interfaces were successfully retrieved.
    if interfaces.is_empty() {
        // If no interfaces are found, return an appropriate error.
        return Err(Error::InterfaceError(crate::errors::InterfaceError::NotFound(
            "No network interfaces found.".into(),
        )));
    }

    // Map the interfaces to their names or MAC addresses, collecting them into a vector.
    let names: Vec<String> = interfaces
        .iter()
        .map(|iface| {
            // Return the interface name on Linux.
            #[cfg(target_os = "linux")]
            {
                iface.name.clone()
            }
            // Return the MAC address of the interface on Windows.
            #[cfg(target_os = "windows")]
            {
                format!("Interface MAC: {}", iface.mac.unwrap_or_default())
            }
            // Return the MAC address of the interface for other systems.
            #[cfg(not(any(target_os = "linux", target_os = "windows")))]
            {
                format!("Interface MAC: {}", iface.mac.unwrap_or_default())
            }
        })
        .collect();

    // Return the vector of interface names.
    Ok(names)
}

#[command(async)]
pub fn start_packet_sending(state_mutex: State<'_, Arc<Mutex<SimPcapState>>>, interface: String) -> Result<SimPcapState, Error> {
    println!("Interface: {interface}");
    let mut state = state_mutex
        .lock()?;
    state.start_simulation()?;
    println!("state: {:?}", state);
    Ok(state.clone())
}

#[command]
pub fn pause_packet_sending(state_mutex: State<'_, Arc<Mutex<SimPcapState>>>) -> Result<SimPcapState, Error> {
    println!("pause_packet_sending");
    let mut state = state_mutex
        .lock()?;
    state.stop_simulation()?;
    println!("state: {:?}", state);
    Ok(state.clone())
}

#[command]
pub fn get_status(state_mutex: State<'_, Arc<Mutex<SimPcapState>>>) -> Result<bool, String> {
    let state = state_mutex
        .lock()
        .map_err(|_| "Failed to acquire lock".to_string())?;
    println!("sim_status: {}", state.sim_status);
    Ok(state.sim_status)
}

#[command]
pub fn update_status(
    state_mutex: State<'_, Arc<Mutex<SimPcapState>>>,
    sim_status: bool,
) -> Result<(), String> {
    let mut state = state_mutex
        .lock()
        .map_err(|_| "Failed to acquire lock".to_string())?;
    state.sim_status = sim_status;
    Ok(())
}
