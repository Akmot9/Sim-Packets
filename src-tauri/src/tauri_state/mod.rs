pub mod sim_packet;
use sim_packet::try_find_interface;
use crate::errors::InterfaceError;

#[derive(Debug)]
pub struct SimPcapState {
    speed: u32,
    loop_state: bool,
    delay: u64,
    ignore_state: bool,
    current_file: Option<String>,
    packet_sended: u32,
    status: Option<String>,
    pub sim_status: bool,
}

// You might want to implement Default to initialize with default values
impl Default for SimPcapState {
    fn default() -> Self {
        Self {
            speed: 1,
            loop_state: false,
            delay: 0,
            ignore_state: false,
            current_file: None,
            packet_sended: 0,
            status: None,
            sim_status: false,
        }
    }
}

impl SimPcapState {
    pub fn start_simulation(&mut self, interface_name: String) -> Result<bool, InterfaceError> {
        // Attempt to find the interface
        let interface = try_find_interface(interface_name)?;
        println!("selcted interface : {}",interface);
        // Update the simulation status
        self.sim_status = true;
        
        // Return the new status
        Ok(self.sim_status)
    }
}
