<template>
  <div
    class="col-start-2 col-end-12 row-start-3 md:col-start-4 md:col-span-1 md:row-start-2"
  >
    <div
      class="bg-sky-100 min-h-full grid grid-cols-9 md:grid-cols-3 justify-center border-x-0 border-t-0 border-2 border-sky-300 shadow-sky-300 shadow md:shadow-md"
    >
      <button
        @click="store.setMyNumber(i)"
        v-for="i in numbers"
        class="p-3 py-5 text-3xl lg:text-5xl xl:text-6xl tfont text-sky-900 transition-colors hover:bg-white delay-100 dark:text-orange-200"
        :class="{'bg-sky-50': isSelectedNumber(i)}"
      >
        {{ i }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSudokuStore } from '@/store/sudokuStore';

let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];
const store = useSudokuStore();

function isSelectedNumber(n: number): boolean {
  if (store.myNumber() == n) {
    return true;
  }
  return false;
}
</script>

<style scoped>

.tfont {
  font-family: TitilliumRegular;
}
</style>
