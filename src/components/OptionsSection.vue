<template>
  <div class="options-section">
    <!-- Paquet Per Packet -->
    <div class="option-item">
      <label for="paquetPerPacket">Debug mod:</label>
      <input
        type="checkbox"
        id="paquetPerPacket"
        v-model="localpaquetDebug"
        @change="updatePaquetDebug"
      />
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import { useSimulationStore } from '../simulationStore';

export default defineComponent({
  setup() {
    const store = useSimulationStore(); // Pinia store
    return { store };
  },
  data() {
    return {
      localpaquetDebug: false, // Valeur locale initialisée
    };
  },
  mounted() {
    // Synchroniser avec l'état Pinia lors du montage du composant
    this.localpaquetDebug = this.store.packet_debug;
  },
  methods: {
    updatePaquetDebug() {
      // Mettre à jour l'état local et dans le store Pinia
      this.store.packet_debug = this.localpaquetDebug;
      console.log('Debug mod:', this.localpaquetDebug);
      console.log('Debug mod state:', this.store.packet_debug);
      this.store.saveStateToTauri();
    },
  },
});
</script>

<style scoped>
.options-section {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 10px;
}

.option-item {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 10px;
}

label {
  font-weight: bold;
}
</style>
