<script setup lang="ts">
import { ElConfigProvider } from "element-plus";
</script>

<template>
  <div id="app">
    <el-config-provider>
      <router-view />
    </el-config-provider>
  </div>
</template>

<style scoped>
html,
body,
#app {
  width: 100vw;
  height: 100vh;
  margin: 0;
  padding: 0;
  background-color: bisque;
  overflow: hidden;
}
</style>