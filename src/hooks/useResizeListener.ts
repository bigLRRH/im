import { onMounted, onBeforeUnmount } from 'vue'
import debounce from 'lodash/debounce'

/**
 * 自定义 Hook：监听窗口大小变化（带防抖）
 * @param callback 原始回调函数
 * @param delay 防抖时间，默认 200ms
 */
export function useResizeListener(callback: () => void, delay = 200) {
    const debouncedCallback = debounce(callback, delay)

    onMounted(() => {
        callback() // 初次调用一次（非防抖）
        window.addEventListener('resize', debouncedCallback)
    })

    onBeforeUnmount(() => {
        window.removeEventListener('resize', debouncedCallback)
    })
}