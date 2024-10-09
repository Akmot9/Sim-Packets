<template>
    <div class="options-section">
      <!-- Speed and loop controls -->
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
    },
    methods: {
      updateOptions() {
        this.$emit('updateOptions', {
          playSpeed: this.playSpeed,
          loopSending: this.loopSending,
          loopCount: this.loopCount,
          delayBetweenLoops: this.delayBetweenLoops,
          ignoreFileError: this.ignoreFileError,
        });
      }
    }
  };
  </script>
  