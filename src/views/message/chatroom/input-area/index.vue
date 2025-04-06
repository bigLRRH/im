<script lang="ts" setup>
import { ref } from 'vue';

const message_input = ref('');
</script>

<template>
    <el-container class="input-area">
        <el-header class="input-area-header">
            （工具栏预留）
        </el-header>
        <el-main class="input-area-body">
            <textarea class="message-input" v-model="message_input" />
        </el-main>
        <el-footer class="input-area-footer">
            <!-- todo 选择按enter发送消息还是按ctrl+enter发送消息 -->
            <el-button type="primary">
                发送
            </el-button>
        </el-footer>
    </el-container>
</template>

<style scoped lang="scss">
.input-area {
    display: flex;
    height: 100%;

    .input-area-header {
        height: 40px;
        width: 100%;

        font-size: 15px;

        background-color: #111;

        display: flex;
        align-items: center;
    }

    .input-area-body {
        flex: 1;
        width: 100%;
        padding: 0;
        justify-content: center;
        align-items: flex-start;

        .message-input {
            width: 100%;
            height: 100%;
            padding-left: 20px;
            padding-right: 10px;
            /* 移除输入框边框 */
            border: none;
            /* 移除聚焦时的外框 */
            outline: none;

            resize: none;

            background-color: #111;
            color: #e8e8e8;
            font-size: 15px;
            box-sizing: border-box;

            text-align: left;
            vertical-align: top;

            overflow-y: auto;
            scrollbar-gutter: stable;
        }
    }

    .input-area-footer {
        height: 55px;
        width: 100%;
        background-color: #111;
        display: flex;
        align-items: center;
        justify-content: flex-end
    }
}
</style>
