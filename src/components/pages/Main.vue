<script setup lang="ts">
import DissertationHelper from "../parts/DissertationHelper.vue"
import { ref } from 'vue'

const value = ref('DissertationHelper')

const options = [
  {
    value: 'DissertationHelper',
    label: '英語論文の要約',
  }
]
</script>

<template>
  <div class="common-layout">
    <el-container>
      <el-header>
        <el-select v-model="value" class="m-2" placeholder="Select" size="large">
          <el-option v-for="item in options" :key="item.value" :label="item.label" :value="item.value" />
        </el-select>
      </el-header>
      <el-main>
        <DissertationHelper v-if="value === `DissertationHelper`"></DissertationHelper>
      </el-main>
    </el-container>
  </div>
</template>

<style scoped></style>
