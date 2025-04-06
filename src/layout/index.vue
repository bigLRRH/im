<script lang="ts" setup>
import Navbar from '@/components/navbar/index.vue'
</script>

<template>
    <el-container class="app-container">
        <el-aside class="navbar">
            <Navbar />
        </el-aside>
        <el-main class="app-main">
            <router-view />
        </el-main>
    </el-container>
</template>

<style lang="scss" scoped>
.app-container {
    display: flex;
    flex-direction: row;
    width: 100%;
    height: 100%;
    overflow: hidden;

    .navbar {
        width: 60px;
        background-color: #202020;
        color: #fff;
        display: flex;
        align-items: center;
        justify-content: center;
        writing-mode: vertical-rl;
        text-align: center;
        overflow: hidden;
    }

    .app-main {
        flex: 1;
        background-color: #111;
        color: #fff;
        overflow: hidden;
        display: flex;
        flex-direction: column;
        height: 100%;
        padding: 0;
    }
}
</style>