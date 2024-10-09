<template>
  <div class="options-section">
    <!-- Play Speed Control -->
    <div class="option-item">
      <label for="playSpeed">Play Speed:</label>
      <input
        type="number"
        id="playSpeed"
        v-model="localPlaySpeed"
        @input="updatePlaySpeed"
        min="0.1"
        step="0.1"
      />
    </div>

    <!-- Loop Sending Control -->
    <div class="option-item">
      <label for="loopSending">Loop Sending:</label>
      <input
        type="checkbox"
        id="loopSending"
        v-model="localLoopSending"
        @change="updateLoopSending"
      />
    </div>

    <!-- Loop Count Control -->
    <div class="option-item" v-if="localLoopSending">
      <label for="loopCount">Loop Count:</label>
      <input
        type="number"
        id="loopCount"
        v-model="localLoopCount"
        @input="updateLoopCount"
        min="1"
        step="1"
      />
    </div>

    <!-- Delay Between Loops -->
    <div class="option-item" v-if="localLoopSending">
      <label for="delayBetweenLoops">Delay Between Loops (ms):</label>
      <input
        type="number"
        id="delayBetweenLoops"
        v-model="localDelayBetweenLoops"
        @input="updateDelayBetweenLoops"
        min="0"
        step="100"
      />
    </div>

    <!-- Ignore File Error -->
    <div class="option-item">
      <label for="ignoreFileError">Ignore File Errors:</label>
      <input
        type="checkbox"
        id="ignoreFileError"
        v-model="localIgnoreFileError"
        @change="updateIgnoreFileError"
      />
    </div>

    <!-- Paquet Per Packet -->
    <div class="option-item">
      <label for="paquetPerPacket">Packet Per Paquet:</label>
      <input
        type="checkbox"
        id="paquetPerPacket"
        v-model="localPaquetPerPacket"
        @change="updatePaquetPerPacket"
      />
    </div>
  </div>
</template>

<script lang="ts">
export default {
  props: {
    playSpeed: Number,
    loopSending: Boolean,
    loopCount: Number,
    delayBetweenLoops: Number,
    ignoreFileError: Boolean,
    paquetPerPacket: Boolean
  },
  data() {
    return {
      localPlaySpeed: this.playSpeed,
      localLoopSending: this.loopSending,
      localLoopCount: this.loopCount,
      localDelayBetweenLoops: this.delayBetweenLoops,
      localIgnoreFileError: this.ignoreFileError,
      localPaquetPerPacket: this.paquetPerPacket
    };
  },
  methods: {
    updatePlaySpeed() {
      this.$emit('update:playSpeed', this.localPlaySpeed);
    },
    updateLoopSending() {
      this.$emit('update:loopSending', this.localLoopSending);
    },
    updateLoopCount() {
      this.$emit('update:loopCount', this.localLoopCount);
    },
    updateDelayBetweenLoops() {
      this.$emit('update:delayBetweenLoops', this.localDelayBetweenLoops);
    },
    updateIgnoreFileError() {
      this.$emit('update:ignoreFileError', this.localIgnoreFileError);
    },
    updatePaquetPerPacket() {
      this.$emit('update:paquetPerPacket', this.localPaquetPerPacket);
    }
  },
  watch: {
    playSpeed(newVal) {
      this.localPlaySpeed = newVal;
    },
    loopSending(newVal) {
      this.localLoopSending = newVal;
    },
    loopCount(newVal) {
      this.localLoopCount = newVal;
    },
    delayBetweenLoops(newVal) {
      this.localDelayBetweenLoops = newVal;
    },
    ignoreFileError(newVal) {
      this.localIgnoreFileError = newVal;
    },
    paquetPerPacket(newVal) {
      this.localPaquetPerPacket = newVal;
    }
  }
};
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
