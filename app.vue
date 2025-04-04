<template>
  <div data-tauri-drag-region class="app">test</div>
</template>

<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
const appWindow = getCurrentWindow();
</script>

<style scoped>
html,
body {
  margin: 0;
  padding: 0;
  background: transparent;
  width: 100px;
  height: 100px;
  border: 1px solid black;
}

.app {
  background-color: red;
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100px;
  height: 100px;
}
</style>
