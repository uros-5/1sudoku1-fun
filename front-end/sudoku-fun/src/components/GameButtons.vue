<template>
  <div class="col-start-9 col-span-2 row-start-3 md:col-start-5 md:row-start-2">
    <div
      class="flex flex-col items-center h-full justify-around bg-sky-100 border-l-0 border-t-0 border-2 border-l-sky-300 border-sky-300 shadow-sky-300 shadow"
    >
      <InlineSvg v-for="i in icons" :src="i" />
    </div>
  </div>
</template>

<script setup lang="ts">
import eraser from "@/assets/gameSvg/MdiEraser.svg";
import resign from "@/assets/gameSvg/BxTired.svg";
import defaultCursor from "@/assets/gameSvg/MdiCursorDefault.svg";
import resetUi from "@/assets/gameSvg/SystemUiconsReset.svg";
import InlineSvg from "vue-inline-svg";

let icons = [eraser, resign, defaultCursor, resetUi];
</script>

<style scoped></style>
