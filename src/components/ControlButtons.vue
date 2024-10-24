<template>
  <div class="control-buttons">
    <button
      :class="[
        'btn',
        isPlaying ? 'btn-warning' : 'btn-primary'
      ]"
      @click="togglePlayPause"
      
    >
      {{ isPlaying ? "Pause" : "Play" }}
    </button>
    <button class="btn btn-secondary" @click="close">Close</button>
  </div>
</template>

<script lang="ts">
import { defineComponent, computed } from 'vue';
import { useSimulationStore } from '../simulationStore';
import { exit } from "@tauri-apps/plugin-process";

export default defineComponent({
  setup() {
    const store = useSimulationStore();

    const isPlaying = computed(() => store.sim_status === 'PLAYING');

    const togglePlayPause = () => {
      if (store.sim_status === 'PLAYING') {
        store.setSimStatus('PAUSED');
      } else {
        store.setSimStatus('PLAYING');
      }
    };

    const close = async () => {
      await exit(1);
    };

    return { isPlaying, togglePlayPause, close };
  },
});
</script>

<style scoped>
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
</style>
