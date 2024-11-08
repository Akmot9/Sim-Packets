use std::{thread, time::Duration};

use crate::errors::{Error, InterfaceError};
use pnet::datalink::{self, Channel, DataLinkSender};
use pcap::Capture;

pub fn try_find_interface(interface_name: String) -> Result<datalink::NetworkInterface, Error> {
    println!("try find interface : {}", interface_name) ;
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name || iface.mac.unwrap().to_string() == interface_name);

    match interface {
        Some(iface) => Ok(iface),
        None => Err(Error::InterfaceError(InterfaceError::NotFound(
            interface_name,
        ))),
    }
}

pub fn sim(interface: datalink::NetworkInterface, file_paths: Vec<String>) -> Result<(), Error> {
    // Configure the packet sender
    let (mut tx, _rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, _rx)) => (tx, _rx),
        Ok(_) => return Err(Error::ChannelError("Unhandled channel type".to_string())),
        Err(e) => return Err(Error::ChannelError(e.to_string())),
    };

    let mut total_packets_read = 0;
    let mut total_packets_sent = 0;

    for file_path in file_paths {
        let (packets_read, packets_sent) = handle_pcap_file(file_path, &mut tx)?;
        total_packets_read += packets_read;
        total_packets_sent += packets_sent;
    }

    println!("Total packets read: {}", total_packets_read);
    println!("Total packets sent: {}", total_packets_sent);

    if total_packets_read != total_packets_sent {
        println!(
            "Warning: Some packets were not sent. {} packets were missed.",
            total_packets_read - total_packets_sent
        );
    }

    Ok(())
}

fn handle_pcap_file(file_path: String, tx: &mut Box<dyn DataLinkSender>) -> Result<(usize, usize), Error> {
    // Attempt to open the pcap file, propagating the error if it fails
    let mut cap = Capture::from_file(file_path).map_err(|_| Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "Failed to open pcap file")))?;

    let mut packets_read = 0;
    let mut packets_sent = 0;

    // Iterate over the packets, print them in hexadecimal, and send them over the network interface
    while let Ok(packet) = cap.next_packet() {
        packets_read += 1;
        print_packet_in_hex(&packet.data);
        if send_packet(tx, packet.data.to_vec()).is_ok() {
            packets_sent += 1;
        }
    }

    Ok((packets_read, packets_sent))
}

// Fonction pour afficher un paquet en hexadécimal
fn print_packet_in_hex(data: &[u8]) {
    for byte in data {
        print!("{:02X} ", byte);
    }
    println!();
}

// Fonction pour envoyer un paquet sur l'interface réseau
fn send_packet(tx: &mut Box<dyn DataLinkSender>, data: Vec<u8>) -> Result<(), Error> {
    tx.build_and_send(1, data.len(), &mut |packet| {
        packet.copy_from_slice(&data);
    }); 
    thread::sleep(Duration::from_micros(1)); // 
    Ok(())
}
