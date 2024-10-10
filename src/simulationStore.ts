import { defineStore } from 'pinia';
import { invoke } from "@tauri-apps/api/core";

interface SimulationState {
  current_file: string | null;  // Using `null` instead of `Option` for TypeScript
  packet_sended: number;
  sim_status: boolean;
  packet_debug: boolean;
}

export const useSimulationStore = defineStore('simulation', {
  // The state mirrors your Rust structure
  state: (): SimulationState => ({
    current_file: null,  // Initialize with null
    packet_sended: 0,
    sim_status: false,
    packet_debug: false,
  }),

  // Actions to update the state
  actions: {
    updateCurrentFile(file: string | null) {
      this.current_file = file;
    },
    incrementPacketSended() {
      this.packet_sended += 1;
    },
    setSimStatus(status: boolean) {
      this.sim_status = status;
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
  getters: {
    isSimulationRunning(state: SimulationState): boolean {
      return state.sim_status;
    },
  },
});
