// Vérification hors-GUI : confirme que le comptage total et la progression du
// compteur `packet_sended` suivent réellement l'envoi des paquets.
// Simple utilitaire de vérification : les panics y sont tolérés.
#![allow(clippy::unwrap_used, clippy::expect_used)]
use std::sync::{Arc, Mutex};

use pnet::datalink;
use sim_packets_lib::tauri_state::{sim_packet::sim, SimPcapState};

fn main() {
    let files = vec![
        "../exmple_pcap/ptpv2.pcap".to_string(),
        "../exmple_pcap/captura.NNTP.cap".to_string(),
    ];

    let state = Arc::new(Mutex::new(SimPcapState::default()));

    // Prend la première interface disponible (loopback en général).
    let interface = datalink::interfaces()
        .into_iter()
        .find(|i| i.is_loopback())
        .or_else(|| datalink::interfaces().into_iter().next())
        .expect("no network interface");
    println!("using interface: {}", interface.name);

    let watcher = Arc::clone(&state);
    let handle = std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(50));
            let s = watcher.lock().unwrap();
            println!(
                "progress: {}/{} (running={})",
                s.packet_sended, s.total_packets, s.sim_status
            );
            if !s.sim_status && s.total_packets > 0 {
                break;
            }
        }
    });

    {
        state.lock().unwrap().sim_status = true;
    }
    match sim(interface, files, 1000, &state) {
        Ok(()) => println!("sim finished ok"),
        Err(e) => println!("sim error: {e}"),
    }
    state.lock().unwrap().sim_status = false;
    let _ = handle.join();

    let s = state.lock().unwrap();
    println!("FINAL: {}/{}", s.packet_sended, s.total_packets);
}
