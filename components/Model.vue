<script setup lang="ts">
import { useGLTF, ScreenSizer } from "@tresjs/cientos";
import { invoke } from '@tauri-apps/api/core';

const path = "/models/test.glb";
const { scene } = await useGLTF(path, { draco: true });

const print = () => {
  console.log("printPdf");
  invoke("print");
};

const printByNumber = (number: number) => {
  console.log("printPdf");
  invoke("printByNumber", { number });
};
</script>

<template>
  <ScreenSizer>
    <primitive :scale="[100, 100, 100]" :object="scene" @click="printByNumber(3)" />
  </ScreenSizer>
</template>