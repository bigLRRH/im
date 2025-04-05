<script lang="ts" setup>
import Searchbar from '@/components/searchbar/index.vue'
</script>

<template>
    <el-container class="sidebar">
        <el-header class="searchbar">
            <Searchbar />
        </el-header>
        <el-main class="thread-list">聊天列表</el-main>
    </el-container>
</template>

<style scoped lang="scss">
.sidebar {
    background-color: #1b1b1b;
    color: #fff;
    flex: 1;
    height: 100%;
    flex-direction: column;
    display: flex;
    overflow: hidden;


    .searchbar {
        height: 64px;
        background-color: #1b1b1b;
        border-bottom: 1px solid #232323;
        display: flex;
        align-items: center;
        padding-left: 16px;
    }

    .thread-list {
        flex: 1;
        overflow-y: auto;
        padding: 8px;
    }
}
</style>
