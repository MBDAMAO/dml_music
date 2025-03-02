<template>
    <div class="h-full p-4 flex flex-col items-center justify-center">
        <!-- 状态显示 -->
        <div class="w-full max-w-sm bg-white rounded-xl shadow-lg p-6 space-y-6">
            <!-- 分数统计 -->
            <div class="flex justify-between text-gray-600">
                <div class="text-center">
                    <p class="text-2xl font-bold">{{ score }}</p>
                    <p class="text-sm">得分</p>
                </div>
                <div class="text-center">
                    <p class="text-2xl font-bold">{{ attempts }}</p>
                    <p class="text-sm">尝试</p>
                </div>
            </div>

            <!-- 当前音符显示 -->
            <div class="text-center space-y-4">
                <div class="h-16 flex items-center justify-center">
                    <transition name="fade">
                        <span v-if="showAnswer" class="text-4xl font-bold" :class="answerClass">
                            {{ currentNote }}
                        </span>
                    </transition>
                </div>

                <!-- 控制按钮 -->
                <div class="flex justify-center space-x-4">
                    <button @click="playNewNote"
                        class="px-6 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors">
                        {{ hasPlayed ? '新音符' : '播放声音' }}
                    </button>
                    <button v-if="hasPlayed" @click="repeatNote"
                        class="px-6 py-2 bg-gray-100 text-gray-600 rounded-lg hover:bg-gray-200 transition-colors">
                        重复播放
                    </button>
                </div>
            </div>

            <!-- 音名选择 -->
            <div class="grid grid-cols-3 gap-3">
                <button v-for="note in notes" :key="note" @click="checkAnswer(note)" :disabled="!hasPlayed"
                    class="p-3 text-lg rounded-lg transition-all" :class="[
                        selectedNote === note
                            ? (isCorrect ? 'bg-green-500 text-white' : 'bg-red-500 text-white')
                            : 'bg-gray-100 hover:bg-gray-200 text-gray-700',
                        hasPlayed ? '' : 'opacity-50 cursor-not-allowed'
                    ]">
                    {{ note }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref } from 'vue';
import { computed } from 'vue';

const notes = ['A', 'A#', 'B', 'C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#'];
const score = ref(0);
const attempts = ref(0);
const currentNote = ref('');
const hasPlayed = ref(false);
const selectedNote = ref('');
const isCorrect = ref(false);
const showAnswer = ref(false);

// 实际使用时需要实现音频播放逻辑
const playNote = () => {
    // 这里需要添加音频播放逻辑
    // 示例：随机选择一个音符
    currentNote.value = notes[Math.floor(Math.random() * notes.length)];
    hasPlayed.value = true;
};

const playNewNote = () => {
    resetState();
    playNote();
};

const repeatNote = () => {
    // 重复播放当前音符
    playNote();
};

const checkAnswer = (selected) => {
    selectedNote.value = selected;
    isCorrect.value = selected === currentNote.value;
    showAnswer.value = true;

    if (isCorrect.value) {
        score.value++;
    }

    attempts.value++;

    // 1秒后重置状态
    setTimeout(() => {
        resetState();
        playNewNote();
    }, 1000);
};

const resetState = () => {
    showAnswer.value = false;
    selectedNote.value = '';
    hasPlayed.value = false;
};

const answerClass = computed(() =>
    isCorrect.value ? 'text-green-500' : 'text-red-500'
);
</script>

<style scoped>
.fade-enter-active,
.fade-leave-active {
    transition: opacity 0.3s;
}

.fade-enter-from,
.fade-leave-to {
    opacity: 0;
}
</style>