<!-- MessageList.vue -->
<script lang="ts" setup>
import { ref, nextTick } from 'vue'

interface Message {
    id: number
    sender: 'self' | 'other'
    content: string
    time: string
}

const messages = ref<Message[]>([])

let idCounter = 1
const containerRef = ref<HTMLDivElement>()

function scrollToBottom() {
    const el = containerRef.value
    if (el) {
        el.scrollTop = el.scrollHeight
    }
}

function addMessage(msg: Omit<Message, 'id'>) {
    messages.value.push({
        ...msg,
        id: idCounter++,
    })

    nextTick(() => {
        scrollToBottom()
    })
}

defineExpose({
    addMessage,
})
</script>

<template>
    <div class="message-list-container" ref="containerRef">
        <div class="message-list">
            <div v-for="msg in messages" :key="msg.id" :class="['message', msg.sender]">
                <div class="message-time">
                    {{ msg.time }}
                </div>
                <div class="message-content">
                    {{ msg.content }}
                </div>
            </div>
        </div>
    </div>
</template>


<style scoped lang="scss">
.message-list-container {
    height: 100%;
    overflow-y: auto;

    .message-list {
        padding: 10px;
        display: flex;
        flex-direction: column;
    }
}

.message {
    max-width: 70%;
    margin-bottom: 10px;
    padding: 8px 8px;
    border-radius: 8px;
    font-size: 14px;
    line-height: 1.4;
    white-space: pre-wrap;
    word-break: break-word;

    &.self {
        align-self: flex-end;
        background-color: #666;
        color: #fff;
    }

    &.other {
        align-self: flex-start;
        background-color: #262626;
        color: #f2f2f2;
    }
}

.message-time {
    font-size: 10px;
    color: #808080;
    margin-bottom: 4px;
    text-align: right;
}
</style>
