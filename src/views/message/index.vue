<script lang="ts" setup>
import { ref, computed } from 'vue'
import { useLayoutStore } from '@/stores/layout'
import { Splitpanes, Pane } from 'splitpanes'
import 'splitpanes/dist/splitpanes.css'
import { useResizeListener } from '@/hooks/useResizeListener.ts'
import Chatroom from '@/views/message/chatroom/index.vue'
import Sidebar from '@/components/sidebar/index.vue'

const sidebarMinWidthPx = 160
const sidebarMaxWidthPx = 320

const layout = useLayoutStore()

const sidebarMinWidthPercent = computed(() => (sidebarMinWidthPx / window.innerWidth) * 100)
const sidebarMaxWidthPercent = computed(() => (sidebarMaxWidthPx / window.innerWidth) * 100)

const sidebarWidthPercent = ref(0)

function clamp(value: number, min: number, max: number) {
    return Math.max(min, Math.min(value, max))
}

function storeSidebarWidth({ prevPane }: any) {
    const width = window.innerWidth
    const px = (prevPane.size / 100) * width
    layout.setSidebarWidthPx(px)
    sidebarWidthPercent.value = prevPane.size
}

function updateSidebarWidthPercent() {
    const width = window.innerWidth
    const clampedWidth = clamp(layout.sidebarWidthPx, sidebarMinWidthPx, sidebarMaxWidthPx)
    layout.setSidebarWidthPx(clampedWidth)
    sidebarWidthPercent.value = (clampedWidth / width) * 100
}

// 初始化
useResizeListener(updateSidebarWidthPercent)
</script>

<template>
    <!-- 拖拽布局 -->
    <splitpanes class="split-wrapper" @resized="storeSidebarWidth">
        <pane class="sidebar-pane" :min-size="sidebarMinWidthPercent" :max-size="sidebarMaxWidthPercent">
            <Sidebar />
        </pane>

        <pane class="chatroom" :size="100 - sidebarWidthPercent" :min-size="100 - sidebarMaxWidthPercent"
            :max-size="100 - sidebarMinWidthPercent">
            <Chatroom />
        </pane>
    </splitpanes>
</template>

<style scoped lang="scss">
.split-wrapper {
    flex: 1;
    height: 100%;
    background-color: #121212;

    .sidebar-pane {
        min-width: 160px;
        max-width: 320px;
        background-color: #121212;
        border-right: 1px solid #232323;
        height: 100%;
        display: flex;
        flex-direction: column; // Ensure proper layout for child elements
        overflow: hidden; // Prevent content overflow
    }

    .chatroom {
        flex: 1;
        height: 100%;
        width: 100%;
        background-color: #111;
        color: #e8e8e8;
        display: flex;
        flex-direction: column;
        overflow: hidden; // Prevent content overflow
    }
}
</style>
