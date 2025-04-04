<script lang="ts" setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { Splitpanes, Pane } from 'splitpanes'
import 'splitpanes/dist/splitpanes.css'
import { useLayoutStore } from '@/stores/layout'

const layout = useLayoutStore()

const sidebarMinWidthPx = 160
const sidebarMaxWidthPx = 320

const sidebarMinWidthPercent = ref(0)
const sidebarMaxWidthPercent = ref(0)
const sidebarWidthPercent = ref(0)
const chatInputHeightPercent = ref(0)

function updateSidebarWidthPercentFromPx() {
    const width = window.innerWidth
    // 约束宽度在允许范围内
    const clampedWidth = Math.max(
        sidebarMinWidthPx,
        Math.min(layout.sidebarWidthPx, sidebarMaxWidthPx)
    )
    layout.setSidebarWidthPx(clampedWidth)
    sidebarWidthPercent.value = (clampedWidth / width) * 100
}

function updateChatInputHeightPercentFromPx() {
    const height = window.innerHeight
    chatInputHeightPercent.value = (layout.chatInputHeightPx / height) * 100
}

function updateLayoutPercentages() {
    const width = window.innerWidth

    // 计算百分比前先约束存储值
    layout.sidebarWidthPx = Math.max(
        sidebarMinWidthPx,
        Math.min(layout.sidebarWidthPx, sidebarMaxWidthPx)
    )

    sidebarMinWidthPercent.value = (sidebarMinWidthPx / width) * 100
    sidebarMaxWidthPercent.value = (sidebarMaxWidthPx / width) * 100

    updateSidebarWidthPercentFromPx()
    updateChatInputHeightPercentFromPx()
}

function storeSidebarWidth({ prevPane }: any) {
    const width = window.innerWidth
    const px = (prevPane.size / 100) * width
    layout.setSidebarWidthPx(px)
    sidebarWidthPercent.value = prevPane.size
}

function storeChatPaneHeight({ prevPane }: any) {
    const height = window.innerHeight
    const px = (prevPane.size / 100) * height
    layout.setChatInputHeightPx(px)
    chatInputHeightPercent.value = prevPane.size
}

onMounted(() => {
    console.log('onMounted')
    updateLayoutPercentages()
    window.addEventListener('resize', updateLayoutPercentages)
})

onBeforeUnmount(() => {
    console.log('onBeforeUnmount')
    window.removeEventListener('resize', updateLayoutPercentages)
})

</script>

<template>
    <el-container class="app-container">
        <el-aside class="navbar">导航栏</el-aside>

        <!-- 拖拽布局 -->
        <splitpanes class="split-wrapper" @resized="storeSidebarWidth">
            <pane class="sidebar-pane" :min-size="sidebarMinWidthPercent" :max-size="sidebarMaxWidthPercent">
                <el-container class="sidebar">
                    <el-header class="sidebar-search">搜索</el-header>
                    <el-main class="thread-list">聊天列表</el-main>
                </el-container>
            </pane>

            <pane class="chatroom-layout" :size="100 - sidebarWidthPercent" :min-size="100 - sidebarMaxWidthPercent"
                :max-size="100 - sidebarMinWidthPercent">
                <el-container class="chatroom-layout">
                    <el-header class="chatroom-header">当前聊天对象</el-header>
                    <splitpanes class="chatroom-body" @resized="storeChatPaneHeight" horizontal>
                        <pane class="message-list">消息显示区域</pane>
                        <pane class="chatroom-input-area" :size="chatInputHeightPercent" min-size="18" max-size="50">
                            输入框 / 工具栏 / 发送按钮
                        </pane>
                    </splitpanes>
                </el-container>
            </pane>
        </splitpanes>
    </el-container>
</template>

<style scoped>
.app-container {
    width: 100%;
    height: 100%;
    display: flex;
}

.navbar {
    width: 60px;
    background-color: #202020;
    color: #fff;
    display: flex;
    align-items: center;
    justify-content: center;
    writing-mode: vertical-rl;
}

.split-wrapper {
    flex: 1;
    height: 100%;
    background-color: #121212;
}

.sidebar-pane {
    min-width: 160px;
    max-width: 320px;
    background-color: #121212;
    border-right: 1px solid #232323;
}

.sidebar {
    background-color: #1b1b1b;
    color: #fff;
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
}

.sidebar-search {
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

.chatroom-layout {
    height: 100%;
    background-color: #111;
    color: #fff;
    display: flex;
    flex-direction: column;
}

.chatroom-header {
    height: 64px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-bottom: 1px solid #262626;
}

.chatroom-body {
    flex: 1;
    height: 100%;
    display: flex;
    flex-direction: column;
}

.message-list {
    flex: 1;
    overflow-y: auto;
}


.chatroom-input-area {
    min-height: 148px;
    max-height: 311px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-top: 1px solid #262626;
}
</style>

<style>
/* 拓宽 splitter 拖拽交互区域 */
.splitpanes__splitter {
    position: relative;
    z-index: 0;
}

/* 可视部分仍是细线，交互区域扩大 */
.splitpanes__splitter::before {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    /* 可视反馈 */
    opacity: 0;
    z-index: 1;
    pointer-events: auto;
}

/* 鼠标悬停时显示热区辅助色 */
.splitpanes__splitter:hover::before {
    opacity: 1;
}

/* 横向拖动区域扩展（上下拖） */
.splitpanes--horizontal>.splitpanes__splitter::before {
    top: -3px;
    bottom: -3px;
    width: 100%;
    cursor: row-resize;
}

/* 纵向拖动区域扩展（左右拖） */
.splitpanes--vertical>.splitpanes__splitter::before {
    left: -3px;
    right: -3px;
    height: 100%;
    cursor: col-resize;
}
</style>