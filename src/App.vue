<template>
  <div class="app-container">
    <div class="select-section">
      <div class="select-group">
        <label for="adapter">Adapter:</label>
        <select v-model="selectedAdapter" id="adapter">
          <option v-for="adapter in adapters" :key="adapter" :value="adapter">{{ adapter }}</option>
        </select>
      </div>
      <div class="file-group">
        <label for="packetFile">Packet File:</label>
        <input type="text" v-model="packetFile" id="packetFile" readonly />
        <button @click="addFiles">Add File(s)</button>
        <button @click="addFolder">Add File Folder</button>
        <button @click="clearFiles">Clear</button>
      </div>
    </div>
    
    <div class="options-section">
      <div class="speed-control">
        <label>Play Speed:</label>
        <input type="range" v-model="playSpeed" min="0.125" max="4" step="0.125" />
      </div>
      <div class="loop-control">
        <input type="checkbox" v-model="loopSending" id="loopSending" />
        <label for="loopSending">Loop Sending:</label>
        <input type="number" v-model="loopCount" :disabled="!loopSending" min="1" />
        <label for="delayBetweenLoops">Delay Between Loops:</label>
        <input type="number" v-model="delayBetweenLoops" min="0" />
        <div>
          <input type="checkbox" v-model="ignoreFileError" id="ignoreFileError" />
          <label for="ignoreFileError">Ignore any file error</label>
        </div>
      </div>
    </div>

    <div class="sending-info-section">
      <p>Current File: {{ currentFile }}</p>
      <p>Packets Sent: {{ packetsSent }}</p>
      <p>Status: {{ status }}</p>
      <progress-bar :progress="progress"></progress-bar>
    </div>

    <div class="control-buttons">
      <button @click="play">Play</button>
      <button @click="pause">Pause</button>
      <button @click="stop">Stop</button>
      <button @click="close">Close</button>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';
import { message } from '@tauri-apps/api/dialog';
import { open } from '@tauri-apps/api/dialog';

export default {
  data() {
    return {
      selectedAdapter: '',
      adapters: [],
      packetFile: [],
      playSpeed: 1,
      loopSending: false,
      loopCount: 1,
      delayBetweenLoops: 1000,
      ignoreFileError: true,
      currentFile: '',
      packetsSent: 0,
      status: 'Please select an adapter and a packet file, Click Play button to start.',
      progress: 0,
    };
  },
  async mounted() {
    invoke('get_interfaces').then((interfaces) => {
      this.adapters = interfaces;
      if (interfaces.length > 0) {
        this.selectedNetInterface = interfaces[interfaces.length - 1]; // Set the last item as default
      }
    }).catch(error => {
      console.error("Failed to load interfaces:", error);
    });
  },
  methods: {
    async addFiles() {
      const dir = await open({
        multiple: true,
        filters: [{
          name: 'Capture File',
          extensions: ['pcap', 'pcapng']
          
        }]
      });
      console.log("App Data Directory: ", dir);
      if (dir) {
        this.packetFile.push(dir); // Add the file path to the array
        console.log("App Data Directory list: ", this.packetFile);
      }
    },
    addFolder() {
      // Logic for adding folder
    },
    clearFiles() {
      this.packetFile = '';
    },
    play() {
      // Logic for starting the packet sending process
    },
    pause() {
      // Logic for pausing the packet sending process
    },
    stop() {
      // Logic for stopping the packet sending process
    },
    close() {
      // Logic for closing the application or interface
    }
  },
};
</script>

<style>
.app-container {
  padding: 20px;
  font-family: Arial, sans-serif;
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
}

input[type="text"], input[type="number"], select {
  margin-right: 10px;
}

button {
  margin-right: 10px;
}

progress-bar {
  width: 100%;
  height: 20px;
  background-color: #f3f3f3;
  border-radius: 5px;
  overflow: hidden;
  margin-top: 10px;
}

progress-bar::after {
  content: '';
  display: block;
  height: 100%;
  width: var(--progress, 0%);
  background-color: #4caf50;
}
</style>
