<template>
    <div class="w-full h-full bg-slate-200 flex p-3 justify-between">
        <!-- 动态渲染路由链接 -->
        <div v-for="route in routes" :key="route.name" class="h-full w-1/3 bg-slate-300 p-1">
            <RouterLink :to="route.path" class="h-full w-full bg-black flex items-center justify-center text-white">
                {{ route.label }}
            </RouterLink>
        </div>
    </div>
</template>

<script setup>
const routes = [
    { name: "home", path: "/menu/home", label: "HOME" },
    { name: "list", path: "/menu/list", label: "功能" }, // 示例路由
    { name: "setting", path: "/menu/setting", label: "设置" }, // 示例路由
];
</script>

<style scoped></style>