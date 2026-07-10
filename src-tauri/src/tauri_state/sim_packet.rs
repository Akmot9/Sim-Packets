use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

use crate::errors::{Error, InterfaceError};
use crate::tauri_state::SimPcapState;
use pcap::Capture;
use pnet::datalink::{self, Channel, DataLinkSender, NetworkInterface};

pub fn try_find_interface(interface_name: String) -> Result<NetworkInterface, Error> {
    println!("try find interface : {}", interface_name);
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().find(|iface| {
        iface.name == interface_name
            || iface
                .mac
                .is_some_and(|mac| mac.to_string() == interface_name)
    });

    match interface {
        Some(iface) => Ok(iface),
        None => Err(Error::Interface(InterfaceError::NotFound(interface_name))),
    }
}

/// Compte le nombre total de paquets sur l'ensemble des fichiers, afin de
/// pouvoir exprimer la progression en pourcentage. Les fichiers illisibles
/// sont ignorés (ils échoueront aussi à l'envoi et n'ajoutent rien au total).
fn count_total_packets(file_paths: &[String]) -> u32 {
    let mut total: u32 = 0;
    for path in file_paths {
        if let Ok(mut cap) = Capture::from_file(path) {
            while cap.next_packet().is_ok() {
                total = total.saturating_add(1);
            }
        }
    }
    total
}

/// Rejoue les paquets des fichiers `file_paths` sur `interface`.
///
/// Met à jour l'état partagé au fur et à mesure : `total_packets` avant
/// l'envoi, puis `packet_sended` à chaque paquet, ce qui permet à la barre de
/// progression du frontend de suivre la simulation en temps réel. S'arrête tôt
/// si l'utilisateur met la simulation en pause (`sim_status == false`).
pub fn sim(
    interface: NetworkInterface,
    file_paths: Vec<String>,
    delay: u64,
    state: &Arc<Mutex<SimPcapState>>,
) -> Result<(), Error> {
    // Premier passage : total de paquets pour la progression.
    let total = count_total_packets(&file_paths);
    {
        let mut s = state.lock()?;
        s.total_packets = total;
        s.packet_sended = 0;
    }

    // Configure the packet sender
    let (mut tx, _rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => return Err(Error::Channel("Unhandled channel type".to_string())),
        Err(e) => return Err(Error::Channel(e.to_string())),
    };

    for file_path in file_paths {
        if !state.lock()?.sim_status {
            break;
        }
        // `false` => l'utilisateur a mis en pause, on arrête la simulation.
        if !send_file(&file_path, &mut tx, delay, state)? {
            break;
        }
    }

    Ok(())
}

/// Envoie tous les paquets d'un fichier. Renvoie `Ok(false)` si la simulation
/// a été mise en pause en cours de route, `Ok(true)` si le fichier est terminé.
fn send_file(
    file_path: &str,
    tx: &mut Box<dyn DataLinkSender>,
    delay: u64,
    state: &Arc<Mutex<SimPcapState>>,
) -> Result<bool, Error> {
    // Attempt to open the pcap file, propagating the error if it fails
    let mut cap = Capture::from_file(file_path)
        .map_err(|_| Error::Io(std::io::Error::other("Failed to open pcap file")))?;

    while let Ok(packet) = cap.next_packet() {
        // Arrêt anticipé si l'utilisateur a mis la simulation en pause.
        if !state.lock()?.sim_status {
            return Ok(false);
        }

        let data = packet.data.to_vec();
        tx.build_and_send(1, data.len(), &mut |buf| {
            buf.copy_from_slice(&data);
        });

        {
            let mut s = state.lock()?;
            s.packet_sended = s.packet_sended.saturating_add(1);
        }

        thread::sleep(Duration::from_micros(delay));
    }

    Ok(true)
}
