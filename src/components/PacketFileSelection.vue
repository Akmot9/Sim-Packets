<template>
  <div class="file-group">
    <label for="packetFiles">Packet File:</label>
    <button class="btn" @click="('addFiles')">Add File(s)</button>
    <button class="btn btn-clear" @click="$emit('clearFiles')">Clear</button>
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
</template>

<script lang="ts">
export default {
  props: {
    packetFiles: {
      type: Array,
      default: () => [],
    },
  },
};
</script>

<style scoped>
.packet-file-table {
  margin-top: 10px;
}

table {
  width: 100%;
  border-collapse: collapse;
}

table,
th,
td {
  border: 1px solid #ddd;
}

th,
td {
  padding: 8px;
  text-align: left;
}

th {
  background-color: #f5f5f5;
}
</style>
