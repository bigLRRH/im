import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

import '@/styles/global.scss'
import '@/styles/splitpanes.scss'

const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

const app = createApp(App);
app.use(router);
app.use(pinia);
app.mount("#app");
