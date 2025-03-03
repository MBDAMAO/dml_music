import { createRouter, createWebHashHistory } from "vue-router";

const router = createRouter({
    history: createWebHashHistory("/"),
    routes: [
        {
            path: "",
            redirect: "/menu",
        },
        {
            path: "/menu",
            name: "menu",
            component: () => import("@/pages/MenuPage.vue"),
            children: [
                {
                    path: "",
                    redirect: "/menu/list",
                },
                {
                    path: "list",
                    name: "list",
                    component: () => import("@/pages/FuncList.vue"),
                },
                {
                    path: "home",
                    name: "home",
                    component: () => import("@/pages/predict.vue"),
                },
                {
                    path: "setting",
                    name: "setting",
                    component: () => import("@/pages/Setting.vue"),
                },
            ],
        },
        {
            path: "/sub",
            name: "sub",
            component: () => import("@/pages/SubPage.vue"),
            children: [
                {
                    path: "",
                    redirect: "/sub/perfect",
                },
                {
                    path: "perfect",
                    name: "perfect",
                    component: () => import("@/pages/Perfect.vue"),
                },
                {
                    path: "tone",
                    name: "tone",
                    component: () => import("@/pages/Tone.vue"),
                },
            ],
        },
    ],
});

export default router;
