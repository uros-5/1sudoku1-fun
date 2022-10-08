<template>
  <main
    class="w-full md:min-h-40 xl:min-h-screen flex flex-col justify-center items-center gap-y-32 md:gap-42 content-around p-9"
  >
    <HomeIcon class="mt-10 w-auto sm:w-4/5 md:w-3/5 lg:w-2/5 xl:w-2/5" />
    <router-link
      to="/game/123"
      class="text-center tfont primary-text rounded-3xl bg-orange-200 text-sky-900 hover:bg-orange-300"
      >Create game</router-link
    >
    <h4 class="text-center tfont primary-text">0 players</h4>
    <span class="hidden fill-cyan90">
      <span class="fill-cyan-900"></span>
      <span class="fill-sky-700"></span>
      <span class="fill-orange-100"></span>
      <span class="fill-sky-200 dark:fill-sky-900"></span>
      <span class="fill-gray-200"></span>
      <span class="bg-gradient-to-r from-sky-200 to-sky-300"></span>
    </span>
  </main>
</template>

<script setup lang="ts">
import HomeIcon from "@/components/HomeIcon.vue";
import { useBackgroundSvgStore } from "@/store/backgroundSvg";
const bg = useBackgroundSvgStore();
</script>

<style scoped>
.tfont {
  font-family: TitilliumThin;
  font-weight: bold;
}
</style>
