<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Factory, Formatter, Renderer, Stave, StaveNote } from 'vexflow'

// 绑定用于渲染 VexFlow 的容器
const vexContainer = ref<HTMLDivElement | null>(null)

onMounted(() => {
  if (vexContainer.value) {
    // 指定一个容器 id
    const containerId = 'vexflow-container'
    vexContainer.value.id = containerId

    // 模板代码
    const div = document.getElementById("vexflow-container")
    const context = new Renderer(div, Renderer.Backends.SVG)
      .resize(800, 200)
      .getContext();
    // 创建 Stave，其在画布上的 x，y 轴分别偏移 10，40，长度 400
    const stave = new Stave(10, 40, 800)
      .addClef("treble") // 添加调号，treble 是 G 谱号
      .setContext(context) // 注入 context
      .draw(); // 然后进行绘制

    const cNote = new StaveNote({ keys: ["C/4"], duration: "2" }) // C4，2 分音符
    const restNote = new StaveNote({ keys: ['B/4'], duration: '2rdd' }) // 二分复附点休止符，B/4 用于标识休止符的位置
    const cMajorTraid = new StaveNote({ keys: ['C/4', 'E/4', 'G/4', 'B/4'], duration: '4' }) // C 大三和弦，四分
    Formatter.FormatAndDraw(context, stave, [cNote, cMajorTraid]);
  }
})
</script>

<template>
  <!-- 使用 Tailwind CSS 设置内边距与背景颜色 -->
  <div class="p-4 bg-white">
    <!-- VexFlow 渲染的容器 -->
    <div ref="vexContainer"></div>
  </div>
</template>

<style scoped>
/* 如有需要可进一步自定义样式 */
</style>
