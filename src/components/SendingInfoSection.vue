<template>
  <div class="sending-info-section">
    <p>
      Current File: <strong>{{ currentFile }}</strong>
    </p>
    <p>
      Packets Sent: <strong>{{ packetsSent }}</strong>
    </p>
    <p>
      Status: <strong>{{ status }}</strong>
    </p>
    <div class="progress-bar">
      <div class="progress-bar-inner" :style="{ width: progress + '%' }"></div>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  props: {
    currentFile: String,
    packetsSent: Number,
    status: String,
    progress: Number,
  },
};
</script>
