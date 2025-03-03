<script setup lang="ts">
import { ref, onMounted } from 'vue'
import Chart from 'chart.js/auto'

// 固定键盘容器高度（单位 px）
const containerHeight = 800

// 定义白键（降序排列，高音在上，低音在下）  
// 范围：上八度为 B4 ~ C4，下八度为 B3 ~ C3，共 14 个白键
const whiteKeys = [
    { note: 'B', octave: 4 },
    { note: 'A', octave: 4 },
    { note: 'G', octave: 4 },
    { note: 'F', octave: 4 },
    { note: 'E', octave: 4 },
    { note: 'D', octave: 4 },
    { note: 'C', octave: 4 },
    { note: 'B', octave: 3 },
    { note: 'A', octave: 3 },
    { note: 'G', octave: 3 },
    { note: 'F', octave: 3 },
    { note: 'E', octave: 3 },
    { note: 'D', octave: 3 },
    { note: 'C', octave: 3 }
]
const numWhiteKeys = whiteKeys.length
const whiteKeyHeight = containerHeight / numWhiteKeys  // 每个白键高度
const blackKeyHeight = whiteKeyHeight * 0.6             // 黑键高度较短

// 定义黑键（降序排列）  
// 注意：在正向（升序）一个八度内，白键顺序为 C, D, E, F, G, A, B，黑键为：C#, D#, (无键), F#, G#, A#  
// 反过来，降序时则为：
// 上八度（对应白键 B4～C4，共 7 个）：  
//   - 黑键 A#4 位于 B4 与 A4之间  
//   - 黑键 G#4 位于 A4 与 G4之间  
//   - 黑键 F#4 位于 G4 与 F4之间  
//   - （E4 与 F4之间无黑键）  
//   - 黑键 D#4 位于 E4 与 D4之间  
//   - 黑键 C#4 位于 D4 与 C4之间  
// 下八度（对应白键 B3～C3）：  
//   - 黑键 A#3 位于 B3 与 A3之间  
//   - 黑键 G#3 位于 A3 与 G3之间  
//   - 黑键 F#3 位于 G3 与 F3之间  
//   - （E3 与 F3之间无黑键）  
//   - 黑键 D#3 位于 E3 与 D3之间  
//   - 黑键 C#3 位于 D3 与 C3之间  

const blackKeysUpperArr = [
    { note: 'A#', octave: 4, between: [0, 1] }, // between 白键 index0(B4) 和 index1(A4)
    { note: 'G#', octave: 4, between: [1, 2] },
    { note: 'F#', octave: 4, between: [2, 3] },
    { note: 'D#', octave: 4, between: [4, 5] },
    { note: 'C#', octave: 4, between: [5, 6] }
]
const blackKeysLowerArr = [
    { note: 'A#', octave: 3, between: [7, 8] },
    { note: 'G#', octave: 3, between: [8, 9] },
    { note: 'F#', octave: 3, between: [9, 10] },
    { note: 'D#', octave: 3, between: [11, 12] },
    { note: 'C#', octave: 3, between: [12, 13] }
]
// 计算黑键在整体容器中的 top 值：  
// 在降序排列中，若黑键位于白键 i 与 i+1 之间，则其中心位置应为 (i+1)*whiteKeyHeight，故 top = (i+1)*whiteKeyHeight - blackKeyHeight/2
const blackKeys = [...blackKeysUpperArr, ...blackKeysLowerArr].map(bk => {
    const i = bk.between[0]
    const center = (i + 1) * whiteKeyHeight
    return {
        note: bk.note,
        octave: bk.octave,
        top: center - blackKeyHeight / 2
    }
})

// 建立音高到物理位置（中心）的映射，用于图表和高亮  
// 对于白键，其中心 = whiteKeyTop + whiteKeyHeight/2；对于黑键，中心 = top + blackKeyHeight/2
const pitchPositions: Record<string, number> = {}
whiteKeys.forEach((wk, index) => {
    const pitch = wk.note + wk.octave
    const center = (index + 0.5) * whiteKeyHeight
    pitchPositions[pitch] = center
})
blackKeys.forEach(bk => {
    const pitch = bk.note + bk.octave
    const center = bk.top + blackKeyHeight / 2
    pitchPositions[pitch] = center
})

// 生成所有音高列表（共 24 个），按屏幕从上到下顺序（高音在上）  
const allPitches = Object.keys(pitchPositions).sort((a, b) => pitchPositions[a] - pitchPositions[b])

// 当前检测到的音高（字符串，如 "C#4"）
const currentPitch = ref('C4')

// Chart.js 部分  
const chartCanvas = ref<HTMLCanvasElement | null>(null)
let lineChart: Chart | null = null

// 固定显示的数据点数与 x 轴标签
const dataLength = 20
const timeLabels = Array.from({ length: dataLength }, (_, i) => `${i + 1}`)
// 初始音高数据随机选自 allPitches，并取其中心位置作为 y 值
const pitchData: number[] = Array.from({ length: dataLength }, () => {
    const randomPitch = allPitches[Math.floor(Math.random() * allPitches.length)]
    return pitchPositions[randomPitch]
})

// 根据最新数据点更新当前音高（取与最新 y 值最近的音高）
function updateCurrentPitch() {
    const latestValue = pitchData[pitchData.length - 1]
    let closestPitch = ''
    let minDiff = Infinity
    for (const p in pitchPositions) {
        const diff = Math.abs(pitchPositions[p] - latestValue)
        if (diff < minDiff) {
            minDiff = diff
            closestPitch = p
        }
    }
    currentPitch.value = closestPitch
}

onMounted(() => {
    updateCurrentPitch()
    if (chartCanvas.value) {
        lineChart = new Chart(chartCanvas.value, {
            type: 'line',
            data: {
                labels: timeLabels,
                datasets: [{
                    label: '音高变化',
                    data: pitchData,
                    borderColor: 'rgb(75, 192, 192)',
                    tension: 0.1,
                    fill: false,
                    pointRadius: 3,
                    spanGaps: true
                }]
            },
            options: {
                maintainAspectRatio: false,
                responsive: true,
                scales: {
                    // y 轴范围设为从最高白键中心到最低白键中心
                    y: {
                        min: whiteKeyHeight / 2,
                        max: (numWhiteKeys - 0.5) * whiteKeyHeight,
                        ticks: {
                            // 可自定义回调显示音高名称，这里简化为空字符串
                            callback: () => ''
                        },
                        grid: {
                            drawBorder: true,
                            color: '#e5e7eb'
                        }
                    },
                    x: {
                        title: { display: false, text: '时间' }
                    }
                }
            }
        })
    }

    let timeCounter = dataLength
    // 每 200ms 更新一次数据，保持数据连续连线
    setInterval(() => {
        const randomPitch = allPitches[Math.floor(Math.random() * allPitches.length)]
        const newValue = pitchPositions[randomPitch]
        pitchData.push(newValue)
        pitchData.shift()
        timeCounter++
        timeLabels.push(`${timeCounter}`)
        timeLabels.shift()
        if (lineChart) {
            lineChart.data.labels = timeLabels
            lineChart.data.datasets[0].data = pitchData
            lineChart.update('none')
        }
        updateCurrentPitch()
    }, 200)
})
import Tone from './Tone.vue'
</script>

<template>
    <!-- 外层容器：若屏幕高度不足，出现滚动条 -->
    <div class="overflow-y-auto">
        <!-- 固定 800px 高度容器 -->
        <div class="flex h-[800px]">
            <!-- 左侧：钢琴键盘（宽度 1/4） -->
            <div class="w-[150px] relative border-r border-gray-300" data-tauri-drag-region>
                <!-- 白键：按降序排列 -->
                <div v-for="(wk, index) in whiteKeys" data-tauri-drag-region :key="wk.note + wk.octave"
                    :style="{ height: whiteKeyHeight + 'px' }"
                    class="border border-gray-500 flex items-center justify-center text-sm relative"
                    :class="(wk.note + wk.octave) === currentPitch ? 'bg-blue-300' : 'bg-white'">
                    <span>{{ wk.note }}{{ wk.octave }}</span>
                </div>
                <!-- 黑键：绝对定位，覆盖在白键之上 -->
                <div class="absolute top-0 left-0 w-full h-full pointer-events-none">
                    <div v-for="bk in blackKeys" :key="bk.note + bk.octave"
                        :style="{ top: bk.top + 'px', height: blackKeyHeight + 'px', width: '60%', left: '40%' }"
                        class="absolute flex items-center justify-center text-xs text-white"
                        :class="currentPitch === (bk.note + bk.octave) ? 'bg-blue-500' : 'bg-black'">
                        {{ bk.note }}{{ bk.octave }}
                    </div>
                </div>
            </div>
            <!-- 右侧：音高变化折线图 -->
            <div class="h-full w-1/2">
                <Tone />
            </div>
        </div>
    </div>
</template>

<style scoped>
/* 如有需要，可进一步调整样式 */
</style>
