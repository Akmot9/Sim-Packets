use pnet::datalink;
use tauri::{command,State};
use crate::tauri_state::SimPcapState;
use std::sync::Mutex;

#[command]
pub fn get_interfaces() -> Vec<String> {
    // Récupère une liste de toutes les interfaces réseau via le module datalink de pnet.
    let interfaces = datalink::interfaces();
    // Log l'action de récupération des interfaces réseau.
    //info!("Récupération des interfaces réseau");

    // Mappe les interfaces à leurs noms ou adresses MAC, les collectant dans un vecteur.
    let  names: Vec<String> = interfaces
        .iter()
        .map(|iface| {
            // Retourne le nom de l'interface sous Linux.
            #[cfg(target_os = "linux")]
            {
                iface.name.clone()
            }
            // Retourne l'adresse MAC de l'interface sous Windows.
            #[cfg(target_os = "windows")]
            {
                format!("Interface MAC: {}", iface.mac.unwrap_or_default())
            }
            // Retourne l'adresse MAC de l'interface pour d'autres systèmes.
            #[cfg(not(any(target_os = "linux", target_os = "windows")))]
            {
                format!("Interface MAC: {}", iface.mac.unwrap_or_default())
            }
        })
        .collect();

    // Retourne le vecteur de noms d'interface.
    names
}

#[command]
pub fn get_status(state_mutex: State<'_, Mutex<SimPcapState>>) -> Result<bool, String> {
    let state = state_mutex.lock().map_err(|_| "Failed to acquire lock".to_string())?;
    Ok(state.sim_status)
}

#[tauri::command]
pub fn update_status(state_mutex: State<'_, Mutex<SimPcapState>>, sim_status: bool) -> Result<(), String> {
    let mut state = state_mutex.lock().map_err(|_| "Failed to acquire lock".to_string())?;
    state.sim_status = sim_status;
    Ok(())
}
