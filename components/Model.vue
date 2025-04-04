<script setup lang="ts">
import { useGLTF, ScreenSizer } from "@tresjs/cientos";

const path = "/models/test.glb";
const { scene } = await useGLTF(path, { draco: true });
</script>

<template>
  <ScreenSizer>
      <primitive :scale="[100, 100, 100]" :object="scene" />
  </ScreenSizer>
</template>
