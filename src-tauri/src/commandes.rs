use crate::errors::Error;
use crate::tauri_state::sim_packet::{sim, try_find_interface};
use crate::tauri_state::SimPcapState;
use pnet::datalink;
use std::sync::{Arc, Mutex};
use tauri::{command, Emitter, State};

#[command(async)]
pub fn get_interfaces(
        window: tauri::Window,
        sim_state_mutex: State<'_, Arc<Mutex<SimPcapState>>>
    ) -> Result<Vec<String>, Error> {
    // Attempt to retrieve the list of all network interfaces via the pnet datalink module.
    let interfaces = datalink::interfaces();

    // Check if interfaces were successfully retrieved.
    if interfaces.is_empty() {
        // If no interfaces are found, return an appropriate error.
        return Err(Error::Interface(crate::errors::InterfaceError::NotFound(
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
            // Émet l'état ~5 fois par seconde pour une barre de progression fluide.
            std::thread::sleep(std::time::Duration::from_millis(200));
            // Emit an event with the SystemState as its payload
            if let Err(e) = window.emit("system_state_update", state.lock()?.clone()) {
                eprintln!("failed to emit system_state_update: {e}");
            }
        }
    });

    // Return the vector of interface names.
    Ok(names)
}

#[command(async)]
pub fn start_packet_sending(
    state_mutex: State<'_, Arc<Mutex<SimPcapState>>>,
    interface: String,
    files: Vec<String>,
    delay: u64,
) -> Result<SimPcapState, Error> {
    println!("Interface: {interface}, files: {files:?}, delay: {delay}");

    // Résout l'interface tout de suite pour que l'erreur remonte à l'appelant.
    let true_interface = try_find_interface(interface)?;

    let state = Arc::clone(&state_mutex);
    {
        let mut s = state.lock()?;
        // Ignore un second démarrage si une simulation tourne déjà.
        if s.sim_status {
            return Ok(s.clone());
        }
        s.packet_sended = 0;
        s.total_packets = 0;
        s.sim_status = true;
    }

    // La simulation tourne dans un thread : la commande rend la main tout de
    // suite et l'avancement est poussé via l'évènement `system_state_update`.
    let thread_state = Arc::clone(&state);
    std::thread::spawn(move || {
        if let Err(e) = sim(true_interface, files, delay, &thread_state) {
            eprintln!("simulation error: {e}");
        }
        if let Ok(mut s) = thread_state.lock() {
            s.sim_status = false;
        }
    });

    let snapshot = state.lock()?.clone();
    Ok(snapshot)
}

#[command]
pub fn pause_packet_sending(
    state_mutex: State<'_, Arc<Mutex<SimPcapState>>>,
) -> Result<SimPcapState, Error> {
    println!("pause_packet_sending");
    // Passe `sim_status` à false : le thread de simulation le voit et s'arrête.
    let mut state = state_mutex.lock()?;
    state.sim_status = false;
    println!("state: {state:?}");
    Ok(state.clone())
}
