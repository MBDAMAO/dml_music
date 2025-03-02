<template>
    <div class="flex flex-col h-full w-full">
        <div class="w-full h-[calc(100%-80px)] overflow-y-auto">
            <RouterView></RouterView>
        </div>
        <div class="w-full h-[80px] fixed bottom-0">
            <Menu></Menu>
        </div>
    </div>
</template>

<script setup>
import Menu from "./Menu.vue"
</script>

<style scoped></style>