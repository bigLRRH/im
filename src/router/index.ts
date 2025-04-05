import { createRouter, createWebHistory } from "vue-router";

const routes = [
    {
        path: "/",
        component: () => import("../layout/index.vue"),
        children: [
            {
                path: "",
                name: "message",
                component: () => import("../views/message/index.vue"),
            }
        ],
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes: routes,
});

export default router