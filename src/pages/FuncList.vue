<template>
    <div class="h-full w-full" data-tauri-drag-region>
        <div class="h-full w-full overflow-y-auto p-2">
            <!-- 动态渲染列表项 -->
            <div v-for="(item, index) in items" :key="index">
                <div class="w-full h-[80px] mb-2 bg-slate-300 rounded-lg flex items-center justify-center cursor-pointer"
                    @click="goto(item)">
                    {{ item }}
                </div>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router'; // 引入 useRouter

const router = useRouter(); // 获取路由实例
const items = ref([1, 2, 3, 4, 5, 6, 7, 8, 9]); // 示例数据

// 跳转函数
function goto(item) {
    // 根据 item 的值跳转到不同的路由
    switch (item) {
        case 1:
            router.push('/sub/perfect'); // 跳转到首页
            break;
        case 2:
            router.push('/menu/about'); // 跳转到关于页
            break;
        case 3:
            router.push('/menu/contact'); // 跳转到联系页
            break;
        default:
            router.push('/menu/default'); // 默认跳转
            break;
    }
}
</script>

<style scoped></style>