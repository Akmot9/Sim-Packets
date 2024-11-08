use crate::{errors::{ Error, InterfaceError}, tauri_state::Moor};
use pcap::Capture;
use pnet::datalink::{self, Channel, DataLinkSender};

use super::SimPcapState;

pub fn try_find_interface(interface_name: String) -> Result<datalink::NetworkInterface, Error> {
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().find(|iface| {
        iface.name == interface_name || iface.mac.unwrap().to_string() == interface_name
    });

    match interface {
        Some(iface) => Ok(iface),
        _none => Err(Error::InterfaceError(InterfaceError::NotFound(
            interface_name,
        ))),
    }
}

pub fn sim(interface: datalink::NetworkInterface, file_paths: Vec<String>, state: &mut SimPcapState) -> Result<(), Error> {
    // Configure the packet sender
    println!("Creating datalink channel for interface: {}", interface.name);
    let (mut tx, _rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, _rx)) => (tx, _rx),
        Ok(_) => {
            
            return Err(Error::ChannelError("err".to_string()))
        },
        Err(e) => {
            return Err(Error::ChannelError(e.to_string()))
        },
    };
    for file_path in file_paths {
        state.update_current_file(file_path.clone());
        handle_pcap_file(file_path, &mut tx, state)?;
    }

    Ok(())
}

fn handle_pcap_file(file_path: String, tx: &mut Box<dyn DataLinkSender>, state: &mut SimPcapState) -> Result<(), Error> {
    // Log the file path being processed
    log::info!("Sim : {}", &file_path);

    // Attempt to open the pcap file, propagating an error if it fails
    let mut cap = Capture::from_file(&file_path).map_err(|_| {
        Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to open pcap file",
        ))
    })?;

    // Iterate over the packets, log the count, print the packet in hex, and send the packet
    while let Ok(packet) = cap.next_packet() {
        // Print each byte of the packet data in hexadecimal format
        let hex_data: String = packet.data.iter().map(|byte| format!("{:02X}", byte)).collect::<Vec<String>>().join(" ");
        log::info!("Packet in hex: {}", hex_data);

        // Send the packet data over the network interface
        if !state.packet_debug {
            send_packet(tx, packet.data.to_vec(), state)?;
        } else {
            log::info!("Packet debug is active, pausing the simulation...");
        
            // Met le statut de simulation à "faux" (pause)
            state.sim_status = Moor::PAUSED;
            // Attendre que l'utilisateur change le statut pour reprendre
            while state.sim_status != Moor::PLAYING {
                // Boucle jusqu'à ce que `sim_status` soit remis à `true` pour reprendre
                std::thread::sleep(std::time::Duration::from_millis(100)); // Petite attente pour éviter une boucle serrée
            }
        }
    }

    Ok(())
}

// Fonction pour envoyer un paquet sur l'interface réseau
fn send_packet(tx: &mut Box<dyn DataLinkSender>, data: Vec<u8>, state: &mut SimPcapState) -> Result<(), Error> {
    tx.build_and_send(1, data.len(), &mut |packet| {
        packet.copy_from_slice(&data);
    });
    state.increment_packet_sended();
    Ok(())
}