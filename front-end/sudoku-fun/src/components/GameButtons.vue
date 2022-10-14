<template>
  <div class="col-start-2 col-span-10 row-start-4 md:col-start-5 md:row-start-2">
    <div
      
      class="p-3 flex flex-row md:flex-col items-center h-full justify-around bg-sky-100 border-l-0 border-t-0 border-2 border-l-sky-300 border-sky-300 shadow-sky-300 shadow md:shadow-md"
    >
      <InlineSvg
      height="55"
      widht="55"
      class="cursor-pointer"
      @click="store.buttonActions()[i]"
       v-for="(v,i) in icons" :src="v" />
    </div>
  </div>
</template>

<script setup lang="ts">
import eraser from "@/assets/gameSvg/MdiEraser.svg";
import resign from "@/assets/gameSvg/BxTired.svg";
import defaultCursor from "@/assets/gameSvg/MdiCursorDefault.svg";
import resetUi from "@/assets/gameSvg/SystemUiconsReset.svg";
import InlineSvg from "vue-inline-svg";
import { useSudokuStore } from "@/store/sudokuStore";

let icons = [eraser, resign, defaultCursor, resetUi];
const store = useSudokuStore();
</script>

<style scoped></style>
