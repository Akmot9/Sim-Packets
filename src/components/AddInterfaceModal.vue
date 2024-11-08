<template>
  <div class="modal-overlay">
    <div class="modal">
      <h2>Add New Network Interface</h2>
      <div class="form-group">
        <label for="interfaceName">Interface Name:</label>
        <input v-model="interfaceName" id="interfaceName" type="text" />
      </div>
      <div class="form-group">
        <label for="interfaceType">Interface Type:</label>
        <input v-model="interfaceType" id="interfaceType" type="text" />
      </div>
      <div class="modal-buttons">
        <button class="btn btn-primary" @click="createInterface">Create</button>
        <button class="btn btn-secondary" @click="$emit('close')">
          Cancel
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { invoke } from "@tauri-apps/api/core";

export default defineComponent({
  data() {
    return {
      interfaceName: "" as string,
      interfaceType: "" as string,
    };
  },
  methods: {
    async createInterface() {
      try {
        await invoke("create_network_interface", {
          name: this.interfaceName,
          type: this.interfaceType,
        });
        this.$emit("interfaceAdded");
        this.$emit("close");
      } catch (error) {
        console.error("Failed to create network interface:", error);
        alert("Error creating interface. Please try again.");
      }
    },
  },
});
</script>

<style>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
}

.modal {
  background: #fff;
  padding: 20px;
  border-radius: 8px;
  width: 300px;
  text-align: center;
}

.form-group {
  margin-bottom: 15px;
  text-align: left;
}

label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
}

input[type="text"] {
  width: 100%;
  padding: 8px;
  border-radius: 4px;
  border: 1px solid #ccc;
}

.modal-buttons {
  display: flex;
  justify-content: space-between;
}
</style>
