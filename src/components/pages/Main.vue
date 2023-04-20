<script setup lang="ts">
import ModeSelector from '../parts/ModeSelector.vue'
</script>

<template>
  <div class="grid_container">
    <div class="selector">
      <ModeSelector></ModeSelector>
    </div>

    <div class="form">
      <h1>form</h1>
    </div>
  </div>
</template>

<style scoped>
.grid_container {
  display: grid;
  height: 100vh;
  width: 100vw;
  grid-template-columns: 1fr 2fr;
  grid-template-rows: 1fr;
  gap: 0px 0px;
  grid-template-areas:
    'selector form';
}
</style>
