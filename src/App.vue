<template>
  <div class="app-container">
    <div class="header">
      <h1>Packet Sender</h1>
    </div>

    <div class="select-section">
      <div class="select-group">
        <label for="adapter">Adapter:</label>
        <select v-model="selectedAdapter" id="adapter">
          <option v-for="adapter in adapters" :key="adapter" :value="adapter">{{ adapter }}</option>
        </select>
      </div>
      <div class="file-group">
        <label for="packetFiles">Packet File:</label>
        <button class="btn" @click="addFiles">Add File(s)</button>
        <button class="btn btn-clear" @click="clearFiles">Clear</button>
      </div>
      <div class="packet-file-table" v-if="packetFiles.length">
        <table>
          <thead>
            <tr>
              <th>#</th>
              <th>File Path</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(file, index) in packetFiles" :key="index">
              <td>{{ index + 1 }}</td>
              <td>{{ file }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <div class="options-section">
      <div class="speed-control">
        <label>Play Speed:</label>
        <input type="range" v-model="playSpeed" min="0.125" max="4" step="0.125" />
        <span>{{ playSpeed }}x</span>
      </div>
      <div class="loop-control">
        <input type="checkbox" v-model="loopSending" id="loopSending" />
        <label for="loopSending">Loop Sending:</label>
        <input type="number" v-model="loopCount" :disabled="!loopSending" min="1" />
        <label for="delayBetweenLoops">Delay Between Loops:</label>
        <input type="number" v-model="delayBetweenLoops" min="0" />
        <span>{{ delayBetweenLoops }} ms</span>
        <div>
          <input type="checkbox" v-model="ignoreFileError" id="ignoreFileError" />
          <label for="ignoreFileError">Ignore any file error</label>
        </div>
      </div>
    </div>

    <div class="sending-info-section">
      <p>Current File: <strong>{{ currentFile }}</strong></p>
      <p>Packets Sent: <strong>{{ packetsSent }}</strong></p>
      <p>Status: <strong>{{ status }}</strong></p>
      <div class="progress-bar">
        <div class="progress-bar-inner" :style="{ width: progress + '%' }"></div>
      </div>
    </div>
    <div class="control-buttons">
      <button
        :class="['btn', isPlaying ? 'btn-warning' : 'btn-primary', { 'btn-disabled': packetFiles.length === 0 }]"
        @click="togglePlayPause"
        :disabled="packetFiles.length === 0"
      >
        {{ isPlaying ? 'Pause' : 'Play' }}
      </button>
      <button class="btn btn-secondary" @click="close">Close</button>
    </div>
  </div>
  <div>v1</div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { open, message } from '@tauri-apps/api/dialog';
import { exit } from '@tauri-apps/api/process';
import { listen } from '@tauri-apps/api/event'

export default {
  data() {
    return {
      selectedAdapter: '',
      adapters: [],
      packetFiles: [],
      playSpeed: 1,
      loopSending: false,
      loopCount: 1,
      delayBetweenLoops: 1000,
      ignoreFileError: true,
      currentFile: '',
      packetsSent: 0,
      status: 'Please select an adapter and a packet file, Click Play button to start.',
      progress: 0,
      isPlaying: false, // Track if currently playing or paused
    };
  },
  async mounted() {
    try {
      const interfaces = await invoke('get_interfaces');
      this.adapters = interfaces;
      if (interfaces.length > 0) {
        this.selectedAdapter = interfaces[interfaces.length - 1];
      }
    } catch (error) {
      console.error('Failed to load interfaces:', error);
    }
    await listen('system_state_update', (event) => {
      this.updateSimulationState(event.payload);
    })
  },
  methods: {
    async addFiles() {
      const files = await open({
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
      // Logic to start packet sending
      invoke('start_packet_sending', { 
        interface: this.selectedAdapter,
        files: this.packetFiles
      
      }).
      then((message) => this.updateSimulationState(message))
      .catch(error => {
        console.error('Failed to start packet sending:', error);
        this.status = 'Error starting packet sending';
        message(error, 
        { title: 'Failed to start packet sending:', 
        type: 'warning'})
      })
    },
    pause() {
      this.isPlaying = false;
      this.status = 'Simulation paused.';
      // Logic to pause the packet sending process
      invoke('pause_packet_sending')
      then((message) => this.updateSimulationState(message))
      .catch(error => {
        console.error('Failed to pause packet sending:', error);
        this.status = ('Failed to pause packet sending:', error);
      });
    },
    updateSimulationState(message) {
    this.packetsSent = message.packet_sended;
    this.isPlaying = message.sim_status;
    this.status = this.isPlaying ? 'Simulation running...' : 'Simulation paused.';
    },
    async close() {
      await exit(1);
    }
  },
};
</script>

<style>
.app-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
  font-family: 'Arial', sans-serif;
  background-color: #f9f9f9;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.header {
  text-align: center;
  margin-bottom: 20px;
}

h1 {
  font-size: 24px;
  color: #333;
}

.select-section, .options-section, .sending-info-section, .control-buttons {
  margin-bottom: 20px;
}

.select-group, .file-group, .speed-control, .loop-control {
  display: flex;
  align-items: center;
  margin-bottom: 10px;
}

label {
  margin-right: 10px;
  font-weight: bold;
  color: #555;
}

input[type="range"], input[type="number"], select {
  margin-right: 10px;
  padding: 5px;
  border-radius: 4px;
  border: 1px solid #ccc;
}

button.btn {
  padding: 10px 20px;
  border-radius: 4px;
  border: none;
  cursor: pointer;
}

button.btn-primary {
  background-color: #4caf50;
  color: #fff;
}

button.btn-warning {
  background-color: #ff9800;
  color: #fff;
}

button.btn-danger {
  background-color: #f44336;
  color: #fff;
}

button.btn-secondary {
  background-color: #fd0606;
  color: #fff;
}

button.btn-clear {
  background-color: #9e9e9e;
  color: #fff;
}

button:hover {
  opacity: 0.9;
}

button.btn-disabled {
  background-color: #cccccc; /* Light gray background */
  color: #666666; /* Dark gray text */
  cursor: not-allowed; /* Show not-allowed cursor when hovering over a disabled button */
  opacity: 0.6; /* Slightly faded appearance */
}

/* Hover effect should be removed or reduced for disabled button */
button.btn-disabled:hover {
  opacity: 0.6; /* Maintain the same opacity on hover */
}

.packet-file-table {
  margin-top: 10px;
}

table {
  width: 100%;
  border-collapse: collapse;
}

table, th, td {
  border: 1px solid #ddd;
}

th, td {
  padding: 8px;
  text-align: left;
}

th {
  background-color: #f5f5f5;
}

.sending-info-section p {
  margin: 5px 0;
  color: #333;
}

.progress-bar {
  width: 100%;
  height: 20px;
  background-color: #e0e0e0;
  border-radius: 5px;
  margin-top: 10px;
  overflow: hidden;
}

.progress-bar-inner {
  height: 100%;
  background-color: #4caf50;
  transition: width 0.2s;
}
</style>
