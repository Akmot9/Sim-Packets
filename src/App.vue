<template>
  <div class="app-container">
    <!-- Header component to display the application title : Packet Sender-->
    <HeaderComponent />
    
    <!-- Section for selecting network adapters and packet files -->
    <div class="select-section">
      <div class="adapter">
        <!-- Adapter selection dropdown with button to add a new interface -->
        <AdapterSelection :adapters="adapters" v-model="selectedAdapter" @addInterface="openAddInterfaceModal" />

        <!-- Modal for adding a new network interface, visible only when triggered -->
        <AddInterfaceModal v-if="showAddInterfaceModal" @close="closeAddInterfaceModal" @interfaceAdded="handleInterfaceAdded" />
      </div>

      <!-- Packet file selection area with buttons to add and clear packet files -->
      <PacketFileSelection :packetFiles="packetFiles" @addFiles="addFiles" @clearFiles="clearFiles" />
    </div>

    <!-- Section for options like play speed, loop control, and file error handling -->
    <OptionsSection 
      :playSpeed="playSpeed" 
      :loopSending="loopSending" 
      :loopCount="loopCount"
      :delayBetweenLoops="delayBetweenLoops"
      :ignoreFileError="ignoreFileError"
      @updateOptions="updateOptions"
    />

    <!-- Section to display information about the current file, packets sent, status, and progress -->
    <SendingInfoSection
      :currentFile="currentFile"
      :packetsSent="packetsSent"
      :status="status"
      :progress="progress"
    />

    <!-- Control buttons for Play/Pause and closing the application -->
    <ControlButtons
      :isPlaying="isPlaying"
      :hasFiles="packetFiles.length > 0"
      @togglePlayPause="togglePlayPause"
      @close="close"
    />
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { exit } from '@tauri-apps/plugin-process';
import { listen } from '@tauri-apps/api/event';
import { error, attachConsole } from '@tauri-apps/plugin-log'; // Pour attacher la console

import HeaderComponent from './components/HeaderComponent.vue';
import AdapterSelection from './components/AdapterSelection.vue';
import PacketFileSelection from './components/PacketFileSelection.vue';
import OptionsSection from './components/OptionsSection.vue';
import SendingInfoSection from './components/SendingInfoSection.vue';
import ControlButtons from './components/ControlButtons.vue';
import AddInterfaceModal from './components/AddInterfaceModal.vue';

export default defineComponent({
  components: {
    HeaderComponent,
    AdapterSelection,
    PacketFileSelection,
    OptionsSection,
    SendingInfoSection,
    ControlButtons,
    AddInterfaceModal,
  },
  data() {
    return {
      selectedAdapter: '' as string,
      adapters: [] as string[],
      packetFiles: [] as string[],
      playSpeed: 1,
      loopSending: false,
      loopCount: 1,
      delayBetweenLoops: 1000,
      ignoreFileError: true,
      currentFile: '' as string,
      packetsSent: 0,
      status: 'Please select an adapter and a packet file, Click Play button to start.' as string,
      progress: 0,
      isPlaying: false,
      showAddInterfaceModal: false,
    };
  },
  async mounted() {
    // Attacher la console pour capturer tous les logs dans Rust
    await attachConsole();

    try {
      const interfaces: string[] = await invoke('get_interfaces');
      this.adapters = interfaces;
      if (interfaces.length > 0) {
        this.selectedAdapter = interfaces[interfaces.length - 1];
      }
    } catch (err: any) {
      console.error('Failed to load interfaces:', err);
      error(`Error loading interfaces: ${err}`);  // Loguer l'erreur dans Rust via Tauri
    }

    // Ecouter les événements système
    await listen('system_state_update', (event: any) => {
      this.updateSimulationState(event.payload);
    });
  },
  methods: {
    handleInterfaceAdded() {
      // code to handle the interfaceAdded event
    },
    openAddInterfaceModal() {
      this.showAddInterfaceModal = true;
    },
    closeAddInterfaceModal() {
      this.showAddInterfaceModal = false;
    },
    async addFiles() {
      const files: string[] | null = await open({
        multiple: true,
        filters: [{ name: 'Capture File', extensions: ['pcap', 'pcapng', 'cap'] }],
      });
      if (files) {
        this.packetFiles.push(...files);
      }
    },
    clearFiles() {
      this.packetFiles = [];
    },
    togglePlayPause() {
      if (this.isPlaying) {
        this.pause();
      } else {
        this.play();
      }
    },
    play() {
      this.isPlaying = true;
      this.status = 'Simulation started...';
      invoke('start_packet_sending', { 
        interface: this.selectedAdapter,
        files: this.packetFiles
      })
      .then((message: any) => this.updateSimulationState(message))
      .catch((err: any) => {
        console.error('Failed to start packet sending:', err);
        error(`Error starting packet sending: ${err}`);  // Loguer l'erreur dans Rust via Tauri
        this.status = `Error starting packet sending: ${err}`;
      });
    },
    
    pause() {
      this.isPlaying = false;
      this.status = 'Simulation paused.';
      invoke('pause_packet_sending')
      .then((message: any) => this.updateSimulationState(message))
      .catch((err: any) => {
        console.log('Failed to pause packet sending:', err);
        
        this.status = 'Failed to pause packet sending.';
      });
    },
    updateSimulationState(message: any) {
      this.currentFile = message.current_file;
      this.packetsSent = message.packet_sended;
      this.isPlaying = message.sim_status;
      this.status = this.isPlaying ? 'Simulation running...' : 'Simulation paused.';
    },

    /**
     * Closes the application with the given exit code.
     * @returns {Promise<void>} - A promise that resolves when the application has been closed.
     */
    async close(): Promise<void> {
      await exit(1);
    },

    updateOptions(updatedOptions: { playSpeed: number; loopSending: boolean; loopCount: number; delayBetweenLoops: number; ignoreFileError: boolean }) {
      this.playSpeed = updatedOptions.playSpeed;
      this.loopSending = updatedOptions.loopSending;
      this.loopCount = updatedOptions.loopCount;
      this.delayBetweenLoops = updatedOptions.delayBetweenLoops;
      this.ignoreFileError = updatedOptions.ignoreFileError;
    }
  },
});
</script>

<style scoped>
  .app-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    max-width: 700px;
    margin: 40px auto; /* Ajouter de l'espace en haut et en bas */
    padding: 30px;
    background-color: #ffffff; /* Fond blanc pour contraster avec le fond général */
    border-radius: 10px; /* Coins arrondis */
    
    /* Ombre plus prononcée pour un effet de profondeur renforcé */
    box-shadow: 0 12px 30px rgba(0, 0, 0, 0.3), /* Ombre principale plus forte */
                0 6px 8px rgba(0, 0, 0, 0.2);   /* Ombre secondaire plus marquée */

    /* Optionnel : Ajout d'une bordure subtile pour renforcer l'effet 3D */
    border: 1px solid #ddd;

    /* Transition douce pour les changements d'état */
    transition: transform 0.3s ease, box-shadow 0.3s ease;
  }
</style>

