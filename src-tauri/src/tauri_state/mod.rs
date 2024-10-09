pub mod sim_packets;
use crate::errors::Error;
use serde::Serialize;
use sim_packets::{sim, try_find_interface};

#[derive(Debug, Serialize, Clone)]
pub struct SimPcapState {
    // speed: u32,
    // loop_state: bool,
    // delay: u64,
    // ignore_state: bool,
    pub current_file: Option<String>,
    pub packet_sended: u32,
    // status: Option<String>,
    pub sim_status: bool,
}

// You might want to implement Default to initialize with default values
impl Default for SimPcapState {
    fn default() -> Self {
        Self {
            // speed: 1,
            // loop_state: false,
            // delay: 0,
            // ignore_state: false,
            current_file: None,
            packet_sended: 0,
            // status: None,
            sim_status: false,
        }
    }
}

impl SimPcapState {
    pub fn start_simulation(
        &mut self,
        interface: String,
        files: Vec<String>,
    ) -> Result<bool, Error> {
        log::info!("Start simulation");
        // Update the simulation status
        self.sim_status = true;

        let true_interface = try_find_interface(interface)?;
        log::info!("flies: {:?}", files);
        sim(true_interface, files, self)?;
        self.sim_status = false;
        // Return the new status
        Ok(self.sim_status)
    }

    pub fn stop_simulation(&mut self) -> Result<bool, Error> {
        log::info!("Stop simulation");
        // Update the simulation status
        self.sim_status = false;
        // Return the new status
        Ok(self.sim_status)
    }

    // Function to increment the packet_sended counter
    pub fn increment_packet_sended(&mut self) {
        self.packet_sended += 1;
        log::info!("Packet sent: {}", self.packet_sended);
    }

    // Function to update the current file
    pub fn update_current_file(&mut self, file_path: String) {
        self.current_file = Some(file_path);
        log::info!("Current file updated: {}", self.current_file.as_ref().unwrap());
    }
}