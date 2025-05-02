<script setup lang="ts">
import { TresCanvas, PerspectiveCamera } from "@tresjs/core";
import { OrbitControls } from "@tresjs/cientos";
import { onKeyStroke } from "@vueuse/core";

const router = useRouter();

onKeyStroke(["r"], (e) => {
  e.preventDefault();
  window.location.reload();
});

onKeyStroke(["ArrowLeft"], (e) => {
  e.preventDefault();
  router.back();
});
</script>

<template>
  <TresCanvas alpha window-size data-tauri-drag-region>
    <!-- <OrbitControls /> -->
    <PerspectiveCamera />
    <TresAmbientLight intensity="100" />
    <TresDirectionalLight intensity="100" />
    <Suspense>
      <Model />
    </Suspense>
  </TresCanvas>
</template>
