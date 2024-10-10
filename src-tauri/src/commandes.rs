use crate::errors::Error;
use crate::tauri_state::SimPcapState;
use pnet::datalink;
use std::sync::{Arc, Mutex};
use tauri::{command, Emitter, State};

#[command(async)]
pub fn get_interfaces(
    window: tauri::Window,
    sim_state_mutex: State<'_, Arc<Mutex<SimPcapState>>>,
) -> Result<Vec<String>, Error> {

    // Attempt to retrieve the list of all network interfaces via the pnet datalink module.
    let interfaces = datalink::interfaces();

    // Check if interfaces were successfully retrieved.
    if interfaces.is_empty() {
        // If no interfaces are found, return an appropriate error.
        return Err(Error::InterfaceError(
            crate::errors::InterfaceError::NotFound("No network interfaces found.".into()),
        ));
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
                iface.mac.unwrap_or_default().to_string()
            }
            // Return the MAC address of the interface for other systems.
            #[cfg(not(any(target_os = "linux", target_os = "windows")))]
            {
                iface.mac.unwrap_or_default().to_string()
            }
        })
        .collect();
    let state = Arc::clone(&sim_state_mutex);
    // Spawn a new thread
    std::thread::spawn(move || -> Result<(), Error> {
        // Create an infinite loop
        loop {
            // Synchronize the state once per second
            std::thread::sleep(std::time::Duration::from_secs(1));
            // Emit an event with the SystemState as its payload
            window
                // Like a good developer you don't use `.unwrap()` on a Result
                .emit("system_state_update", state.lock()?.clone())
                .unwrap();
        }
    });

    log::info!("Interfaces detected: {:?}", names);
    // Return the vector of interface names.
    Ok(names)
}

#[command(async)]
pub fn start_packet_sending(
    state_mutex: State<'_, Arc<Mutex<SimPcapState>>>,
    interface: String,
    files: Vec<String>,
) -> Result<SimPcapState, Error> {
    log::info!("Interface choosed: {interface}");
    let mut state = state_mutex.lock()?;
    state.start_simulation(interface, files)?;
    log::info!("state: {:?}", state);
    Ok(state.clone())
}

#[command(async)]
pub fn pause_packet_sending(
    state_mutex: State<'_, Arc<Mutex<SimPcapState>>>,
) -> Result<SimPcapState, Error> {
    log::info!("pause_packet_sending");
    let mut state = state_mutex.lock()?;
    state.stop_simulation()?;
    log::info!("state: {:?}", state);
    Ok(state.clone())
}
#[command(async)]
pub fn resume_packet_sending(
    state_mutex: State<'_, Arc<Mutex<SimPcapState>>>,
) -> Result<SimPcapState, Error> {
    log::info!("resume_packet_sending");
    let mut state = state_mutex.lock()?;
    state.resume_simulation()?;
    log::info!("state: {:?}", state);
    Ok(state.clone())
}

#[command(async)]
pub fn update_simulation_state(state_mutex: State<'_, Arc<Mutex<SimPcapState>>>, pinia_state: SimPcapState) -> Result<SimPcapState, Error> {
    println!("update_simulation_state");
    println!("pinia_state: {:?}", pinia_state);
    let mut state = state_mutex.lock()?;
    state.update_sate(pinia_state) ;
    Ok(state.clone())
}