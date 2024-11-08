<template>
  <div class="select-group">
    <label for="adapter">Adapter:</label>
    <select v-model="selectedAdapter" id="adapter">
      <option v-for="adapter in adapters" :key="adapter" :value="adapter">
        {{ adapter }}
      </option>
    </select>
    <button class="btn btn-add-adapter" @click="$emit('addInterface')">
      +
    </button>
  </div>
</template>

<script lang="ts">
export default {
  props: {
    adapters: Array,
    modelValue: String,
  },
  computed: {
    selectedAdapter: {
      get() {
        return this.modelValue;
      },
      set(value: string) {
        this.$emit("update:modelValue", value);
      },
    },
  },
};
</script>
