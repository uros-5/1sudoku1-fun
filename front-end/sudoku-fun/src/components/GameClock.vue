<template>
  <div
    class="col-start-1 col-span-3 row-start-1 md:col-start-2 md:col-span-1 md:row-start-4 md:p-5"
  >
    <div class="flex justify-center">
      <p
        class="text-lg sm:text-3xl cursor-default tfont px-2 md:p-5 bg-sky-100 dark:bg-sky-900 rounded shadow-sky-400 shadow-sm lg:shadow-lg text-sky-900 dark:text-orange-200"
        :class="{'bg-red-400 dark:bg-sky-200':store.isHurry()}"
      >
        {{ minute() }}:{{ second() }}
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import {useSudokuStore } from "@/store/sudokuStore";

const store = useSudokuStore();

function minute(): string {
  return store.$state.clock.currentMin;
}

function second(): string {
  return store.$state.clock.currentSec;
}
</script>

<style scoped>
.tfont {
  font-family: OpenSans;
  font-weight: bold;
}
</style>
