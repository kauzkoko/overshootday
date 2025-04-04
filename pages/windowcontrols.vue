<template>
	<div data-tauri-drag-region class="app">
	  <div @click="appWindow.minimize()" class="bg-red-500 p-2 rounded-md">mini</div>
	  <div @click="appWindow.maximize()" class="bg-blue-500 p-2 rounded-md">maxi</div>
	  <div @click="appWindow.close()" class="bg-green-500 p-2 rounded-md">close</div>
	</div>
  </template>
  
  <script setup lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  const appWindow = getCurrentWindow();
  </script>

<style scoped>
.app {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100vw;
  height: 100vh;
}
</style>