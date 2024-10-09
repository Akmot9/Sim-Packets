<template>
  <header class="header">
    <h1>{{ title }}</h1>
  </header>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  props: {
    title: {
      type: String,
      default: "Packet Sender",
    },
  },
});
</script>

<style scoped>
.header {
  text-align: center;
  margin-bottom: 20px;
}

h1 {
  font-size: 24px;
  color: #333;
  margin: 0;
}
</style>
