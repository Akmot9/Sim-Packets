pub mod sim_packet;
use crate::errors::Error;
use serde::Serialize;
use sim_packet::try_find_interface;

#[derive(Debug, Serialize, Clone)]
pub struct SimPcapState {
    // speed: u32,
    // loop_state: bool,
    // delay: u64,
    // ignore_state: bool,
    // current_file: Option<String>,
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
            // current_file: None,
            packet_sended: 0,
            // status: None,
            sim_status: false,
        }
    }
}

impl SimPcapState {
    pub fn start_simulation(&mut self) -> Result<bool, Error> {
        // Update the simulation status
        self.sim_status = true;

        // Return the new status
        Ok(self.sim_status)
    }

    pub fn stop_simulation(&mut self) -> Result<bool, Error> {
        // Update the simulation status
        self.sim_status = false;
        // Return the new status
        Ok(self.sim_status)
    }
}
