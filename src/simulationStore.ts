import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/core";

export type MoorState = 
    'INIT' | 
    'PLAYING' |
    'RUNNING' | 
    'PAUSED' | 
    'STOPPED';

interface SimulationState {
  current_file: string | null;  // Using `null` instead of `Option` for TypeScript
  packet_sended: number;
  sim_status: MoorState;
  packet_debug: boolean;
}

export const useSimulationStore = defineStore('simulation', {
  // The state mirrors your Rust structure
  state: (): SimulationState => ({
    current_file: null,  // Initialize with null
    packet_sended: 0,
    sim_status: 'INIT' as MoorState,
    packet_debug: false,
  }),

  // Actions to update the state
  actions: {
    updateState(updatedState: SimulationState) {
      this.sim_status = updatedState.sim_status;
      this.packet_debug = updatedState.packet_debug;
      console.info(`State successfully updated to ${updatedState.sim_status}. Debug: ${updatedState.packet_debug}`);
      console.info(`State successfully updated to ${this.sim_status as MoorState}. Debug: ${this.packet_debug}`);
      this.loadStateFromTauri(updatedState);
      this.saveStateToTauri();
    },
    updateCurrentFile(file: string | null) {
      this.current_file = file;
    },
    incrementPacketSended() {
      this.packet_sended += 1;
    },
    setSimStatus(status: MoorState) {
      this.sim_status = status;
      this.updateState({
        sim_status: status,
        current_file: null,
        packet_sended: 0,
        packet_debug: false
      });
    },
    togglePacketDebug() {
      this.packet_debug = !this.packet_debug;
    },
    setPacketDebug(debug: boolean) {
      this.packet_debug = debug;
    },
    async loadStateFromTauri(state_tauri: SimulationState) {
        this.current_file = state_tauri.current_file;
        this.packet_sended = state_tauri.packet_sended;
        this.sim_status = state_tauri.sim_status;
        this.packet_debug = state_tauri.packet_debug;
    },
    async saveStateToTauri() {
        console.log('saveStateToTauri');
        await invoke('update_simulation_state', {
          piniaState: {
            current_file: this.current_file,
            packet_sended: this.packet_sended,
            sim_status: this.sim_status,
            packet_debug: this.packet_debug
          }
        });
    }
  },
  // Optionally, use getters to derive some values from the state
});
