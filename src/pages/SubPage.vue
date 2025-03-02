<template>
    <div class="h-full w-full flex flex-col">
        <!-- 顶部栏 -->
        <div data-tauri-drag-region class="w-full h-[60px] bg-red-200 top-0 fixed flex items-center p-2">
            <!-- 返回按钮 -->
            <div class="h-12 w-12 bg-black flex items-center justify-center text-white cursor-pointer" @click="goBack">
                ←
            </div>
        </div>
        <!-- 内容区域 -->
        <div class="w-full h-[calc(100%-60px)] mt-[60px] overflow-y-auto">
            <RouterView></RouterView>
        </div>
    </div>
</template>

<script setup>
import { useRouter } from 'vue-router'; // 引入 useRouter

const router = useRouter(); // 获取路由实例

// 返回上一级页面
function goBack() {
    router.go(-1); // 返回上一级页面
}
</script>

<style scoped></style>