<!-- Chatroom.vue -->
<script lang="ts" setup>
import { ref } from 'vue'
import { useLayoutStore } from '@/stores/layout'
import { useResizeListener } from '@/hooks/useResizeListener.ts'
import { Splitpanes, Pane } from 'splitpanes'
import 'splitpanes/dist/splitpanes.css'
import InputArea from './input-area/index.vue'
import MessageList from './message-list/index.vue'

const chatInputHeightPercent = ref(0)
const layout = useLayoutStore()

function storeChatPaneHeight({ prevPane }: any) {
    const height = window.innerHeight
    const px = (prevPane.size / 100) * height
    layout.setChatInputHeightPx(px)
    chatInputHeightPercent.value = prevPane.size
}

function updateChatInputHeightPercent() {
    const height = window.innerHeight
    chatInputHeightPercent.value = (layout.chatInputHeightPx / height) * 100
}

useResizeListener(updateChatInputHeightPercent)
</script>

<template>
    <el-container class="chatroom-container">
        <el-header class="chatroom-header">当前聊天对象</el-header>
        <splitpanes class="chatroom-body" @resized="storeChatPaneHeight" horizontal>
            <pane class="message-list">
                <!-- 消息显示区域 -->
                <MessageList />
            </pane>
            <pane class="input-area" :size="chatInputHeightPercent" min-size="18" max-size="50">
                <!-- 输入区域 -->
                <InputArea />
            </pane>
        </splitpanes>
    </el-container>
</template>

<style scoped lang="scss">
.chatroom-container {
    flex: 1;
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
}

.chatroom-header {
    height: 64px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-bottom: 1px solid #262626;
    overflow: hidden;
}

.chatroom-body {
    flex: 1;
    height: 100%;
    display: flex;
    flex-direction: column;
    background-color: #111;
    overflow: hidden;
}

.message-list {
    flex: 1;
    height: 100%;
    background-color: #111;
    overflow: hidden;
}

.input-area {
    min-height: 148px;
    max-height: 311px;
    display: flex;
    border-top: 1px solid #262626;
    overflow: hidden;
}
</style>
