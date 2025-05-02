<template>
    <div class="w-screen h-screen flex justify-center items-center bg-white">
        <div class="grid grid-cols-5 gap-2 w-full h-full m-2 pb-2">
            <button class="text-center bg-gray-50 rounded-md p-2 flex justify-center items-center" v-for="i in 20"
                :key="i" @click="printByNumber(i)" :class="{ 'bg-yellow-200': currentNumber === i }">
                <div class="text-20px">{{ i }}</div>
            </button>
        </div>
    </div>
    <div class="fixed left-50vw top-50vh -translate-x-1/2 -translate-y-1/2 flex items-center justify-center flex-col">
        <button class="sm:text-60px text-30px rounded-md p-2 bg-gray-200 mix-blend-difference whitespace-nowrap"
            @click="printSelected">PRINT SELECTED</button>
        <div class="bg-yellow-200 text-center w-50% p-1 border-solid border-1px border-gray">Press Spacebar</div>
    </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

const currentNumber = ref(1);

const printSelected = () => {
    console.log("printPdf");
    invoke("printByNumber", { number: currentNumber.value });
    currentNumber.value++;
    if (currentNumber.value > 20) {
        currentNumber.value = 1;
    }
};

const printByNumber = (number: number) => {
    console.log("printPdf");
    currentNumber.value = number;
};

onKeyStroke([" ", "Space"], (e) => {
    e.preventDefault();
    console.log("printSelected");
    printSelected();
});

onKeyStroke(["ArrowLeft"], (e) => {
    e.preventDefault();
    currentNumber.value--;
    if (currentNumber.value < 1) {
        currentNumber.value = 20;
    }
});

onKeyStroke(["ArrowRight"], (e) => {
    e.preventDefault();
    currentNumber.value++;
    if (currentNumber.value > 20) {
        currentNumber.value = 1;
    }
});
</script>