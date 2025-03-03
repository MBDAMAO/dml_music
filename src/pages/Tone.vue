<template>
    <div class="h-full w-full">
        <canvas ref="pitchChartCanvas" class="w-full h-full" @wheel="handleScroll" @touchstart="handleTouchStart"
            @touchmove="handleTouchMove">
        </canvas>
    </div>
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { ref, onMounted, onUnmounted } from 'vue';

const currentPitch = ref(0);
const currentNote = ref('');
const pitchHistory = ref<number[]>([]); // 存储音高历史数据
const maxHistoryLength = 200; // 历史数据最大长度
const isDetecting = ref(false);
const pitchChartCanvas = ref<HTMLCanvasElement | null>(null);
let ctx: CanvasRenderingContext2D | null = null;

// 固定纵坐标范围
const minPitch = 50;
const maxPitch = 2000;

// 纵坐标偏移量 (用于滑动查看不同音高区域)
const yOffset = ref(0);
const scrollSpeed = 30; // 滚动步长
let touchStartY = 0;

const freqToNote = (freq: number) => {
    if (freq === 0) return 'N/A';
    const A4 = 440;
    const notes = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];
    const halfSteps = Math.round(12 * Math.log2(freq / A4)) + 57;
    const octave = Math.floor(halfSteps / 12);
    const noteIndex = halfSteps % 12;
    return `${notes[noteIndex]}${octave}`;
};

const startDetection = async () => {
    try {
        await invoke('start_pitch_detection');
        isDetecting.value = true;
    } catch (error) {
        console.error('启动音高检测失败:', error);
    }
};

const stopDetection = async () => {
    try {
        await invoke('stop_pitch_detection');
        isDetecting.value = false;
    } catch (error) {
        console.error('停止音高检测失败:', error);
    }
};

const toggleDetection = () => {
    if (isDetecting.value) {
        stopDetection();
    } else {
        startDetection();
    }
};

// 处理鼠标滚动调整 yOffset
const handleScroll = (event: WheelEvent) => {
    yOffset.value += event.deltaY > 0 ? scrollSpeed : -scrollSpeed;
    drawPitchChart();
};

// 处理触摸滑动
const handleTouchStart = (event: TouchEvent) => {
    touchStartY = event.touches[0].clientY;
};

const handleTouchMove = (event: TouchEvent) => {
    const touchMoveY = event.touches[0].clientY;
    yOffset.value += (touchStartY - touchMoveY) * 0.5;
    touchStartY = touchMoveY;
    drawPitchChart();
};

const mouseX = ref<number | null>(null);
const mouseY = ref<number | null>(null);
const hoverPitch = ref<number | null>(null);
const hoverNote = ref<string | null>(null);

const handleMouseMove = (event: MouseEvent) => {
    if (!pitchChartCanvas.value) return;

    const rect = pitchChartCanvas.value.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const width = pitchChartCanvas.value.width;

    // 找到最接近鼠标的音高数据点
    const index = Math.round((x / width) * pitchHistory.value.length);
    if (index >= 0 && index < pitchHistory.value.length) {
        const pitch = pitchHistory.value[index];
        hoverPitch.value = pitch;
        hoverNote.value = freqToNote(pitch);
        mouseX.value = x;
        mouseY.value = pitch;
    } else {
        hoverPitch.value = null;
        hoverNote.value = null;
        mouseX.value = null;
        mouseY.value = null;
    }

    drawPitchChart(); // 重新绘制图表
};

const drawPitchChart = () => {
    if (!ctx || !pitchChartCanvas.value) return;

    const width = pitchChartCanvas.value.width;
    const height = pitchChartCanvas.value.height;

    // 清空画布
    ctx.clearRect(0, 0, width, height);

    // 计算纵坐标比例
    const pitchRange = maxPitch - minPitch;
    const yScale = height / pitchRange;

    // 画刻度线
    ctx.fillStyle = 'white';
    ctx.font = '14px Arial';
    ctx.strokeStyle = '#888';
    ctx.lineWidth = 1;
    for (let pitch = minPitch; pitch <= maxPitch; pitch += 50) {
        const y = height - ((pitch - minPitch - yOffset.value) * yScale);
        if (y >= 0 && y <= height) {
            ctx.fillText(`${pitch} Hz`, 5, y);
            ctx.beginPath();
            ctx.moveTo(50, y);
            ctx.lineTo(width, y);
            ctx.stroke();
        }
    }

    // 绘制音高变化连线（浅色）
    ctx.beginPath();
    ctx.strokeStyle = '#93c5fd'; // 浅蓝色
    ctx.lineWidth = 2;

    pitchHistory.value.forEach((pitch, index) => {
        const x = index * (width / pitchHistory.value.length);
        const y = height - ((pitch - minPitch - yOffset.value) * yScale);

        if (index === 0) {
            ctx.moveTo(x, y);
        } else {
            ctx.lineTo(x, y);
        }
    });

    ctx.stroke();

    // 绘制数据点（深色）
    ctx.fillStyle = '#1e40af'; // 深蓝色
    pitchHistory.value.forEach((pitch, index) => {
        const x = index * (width / pitchHistory.value.length);
        const y = height - ((pitch - minPitch - yOffset.value) * yScale);
        ctx.beginPath();
        ctx.arc(x, y, 3, 0, Math.PI * 2);
        ctx.fill();
    });

    // 画十字准星
    if (mouseX.value !== null && mouseY.value !== null) {
        const hoverY = height - ((mouseY.value - minPitch - yOffset.value) * yScale);

        ctx.strokeStyle = '#ff0000';
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.moveTo(mouseX.value, 0);
        ctx.lineTo(mouseX.value, height);
        ctx.stroke();

        ctx.beginPath();
        ctx.moveTo(0, hoverY);
        ctx.lineTo(width, hoverY);
        ctx.stroke();
    }
};

// 监听鼠标移动
onMounted(() => {
    pitchChartCanvas.value?.addEventListener('mousemove', handleMouseMove);
});

onUnmounted(() => {
    pitchChartCanvas.value?.removeEventListener('mousemove', handleMouseMove);
});

// 监听音高数据
onMounted(async () => {
    toggleDetection()
    if (pitchChartCanvas.value) {
        ctx = pitchChartCanvas.value.getContext('2d');
        const parent = pitchChartCanvas.value.parentElement;
        if (parent) {
            pitchChartCanvas.value.width = parent.clientWidth; // 让 canvas 和父元素一致
            pitchChartCanvas.value.height = parent.clientHeight;
        }
    }

    await listen<number>('pitch_data', (event) => {
        const newPitch = Number(event.payload);
        currentPitch.value = newPitch;
        currentNote.value = freqToNote(newPitch);

        // 记录历史音高数据
        if (newPitch !== 0) {
            pitchHistory.value.push(newPitch);
            if (pitchHistory.value.length > maxHistoryLength) {
                pitchHistory.value.shift();
            }
            drawPitchChart(); // 更新图表
        }
    });
});

onUnmounted(() => {
    ctx = null;
});
</script>
