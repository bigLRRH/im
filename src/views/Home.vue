<script lang="ts" setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
    <el-container class="app-container">
        <!-- 导航栏 -->
        <el-aside class="main-nav">导航栏</el-aside>
        <el-container>
            <!-- 侧边栏 -->
            <el-aside class="chat-sidebar">
                侧边栏
                <br />
                宽度160px到320px
            </el-aside>
            <el-container class="chat-content">
                <!-- 头部 -->
                <el-header class="chat-header">
                    头部
                    <br />
                    高度64px
                </el-header>
                <!-- 主内容 -->
                <el-main class="message-list">
                    消息显示
                    <el-input v-model="name" placeholder="输入你的名字" class="input-box" />
                    <el-button type="primary" @click="greet">Greet</el-button>
                    <p>{{ greetMsg }}</p>
                </el-main>
                <!-- 底部 -->
                <el-footer class="message-input-area">
                    消息输入与发送
                    <br />
                    高度148px到311px
                </el-footer>
            </el-container>
        </el-container>
    </el-container>
</template>

<style scoped>
/* 主要布局 */
.app-container {
    width: 100%;
    height: 100%;
    display: flex;
}

/* 导航栏 */
.main-nav {
    width: 60px;
    background-color: #202020;
    color: #fff;
    display: flex;
    align-items: center;
    justify-content: center;
    /* 竖向文字 */
    writing-mode: vertical-rl;
}

/* 侧边栏 */
.chat-sidebar {
    min-width: 160px;
    max-width: 320px;
    background-color: #1b1b1b;
    color: #fff;
    display: flex;
    align-items: center;
    justify-content: center;
    /* 避免被挤压 */
    flex-shrink: 0;
    overflow: auto;
}

/* 内容容器 */
.chat-content {
    display: flex;
    flex-direction: column;
    flex: 1;
    /* 让主内容填充剩余空间 */
}

/* 头部 */
.chat-header {
    height: 64px;
    background-color: #111111;
    color: #fff;
    display: flex;
    align-items: center;
    justify-content: center;
    border-bottom: 1px solid #262626;
}

/* 主内容 */
.message-list {
    flex: 1;
    /* 让主内容填充剩余空间 */
    background-color: #111111;
    color: #fff;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

/* 输入框样式 */
.input-box {
    margin-bottom: 10px;
    width: 200px;
}

/* 底部 */
.message-input-area {
    min-height: 148px;
    max-height: 311px;
    background-color: #111111;
    color: #fff;
    display: flex;
    align-items: center;
    justify-content: center;
    border-top: 1px solid #262626;
}
</style>