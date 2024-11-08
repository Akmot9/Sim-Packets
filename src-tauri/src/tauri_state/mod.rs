pub mod sim_packets;
use crate::errors::Error;
use serde::{Deserialize, Serialize};
use sim_packets::{sim, try_find_interface};

#[derive(Debug, Serialize, Clone, Deserialize, Copy, PartialEq)]
pub enum Moor {
    INIT,
    PLAYING,
    PAUSED,
    STOPPED,
    COMPLETED,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct SimPcapState {
    // speed: u32,
    // loop_state: bool,
    // delay: u64,
    // ignore_state: bool,
    pub current_file: Option<String>,
    pub packet_sended: u32,
    // status: Option<String>,
    pub sim_status: Moor,
    pub packet_debug: bool,
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
            sim_status: Moor::INIT,
            packet_debug: false,
        }
    }
}

impl SimPcapState {
    pub fn start_simulation(
        &mut self,
        interface: String,
        files: Vec<String>,
    ) -> Result<Moor, Error> {
        log::info!("Start simulation");
        // Update the simulation status
        self.sim_status = Moor::PLAYING;

        let true_interface = try_find_interface(interface)?;
        log::info!("flies: {:?}", files);
        sim(true_interface, files, self)?;
        self.sim_status = Moor::COMPLETED;
        // Return the new status
        Ok(self.sim_status)
    }

    pub fn stop_simulation(&mut self) -> Result<Moor, Error> {
        log::info!("Stop simulation");
        // Update the simulation status
        self.sim_status = Moor::STOPPED;
        // Return the new status
        Ok(self.sim_status)
    }

    pub fn resume_simulation(&mut self) -> Result<Moor, Error> {
        log::info!("Resume simulation");
        // Update the simulation status
        self.sim_status = Moor::PLAYING;
        // Return the new status
        Ok(self.sim_status)
    }

    // Function to increment the packet_sended counter
    pub fn increment_packet_sended(&mut self) {
        self.packet_sended += 1;
        log::info!("Packet sent: {}", self.packet_sended);
    }

    pub fn update_current_file(&mut self, file_path: String) {
        self.current_file = Some(file_path);
        log::info!("Current file updated: {}", self.current_file.as_ref().unwrap());
    }

    // Function to update the current file
    pub fn update_sate(&mut self, pinia_sate: SimPcapState) {
        self.current_file = pinia_sate.current_file;
        self.packet_sended = pinia_sate.packet_sended;
        self.sim_status = pinia_sate.sim_status;
        self.packet_debug = pinia_sate.packet_debug;

        log::info!("Current file updated: {:?}", self);
    }


}