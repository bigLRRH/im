import { defineStore } from 'pinia'

export const useLayoutStore = defineStore('layout', {
    state: () => ({
        sidebarWidthPx: 240,
        chatInputHeightPx: 180,
    }),
    actions: {
        setSidebarWidthPx(px: number) {
            this.sidebarWidthPx = px
        },
        setChatInputHeightPx(px: number) {
            this.chatInputHeightPx = px
        },
    },
    persist: true,
})
