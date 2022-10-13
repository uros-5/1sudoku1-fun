<script setup lang="ts">
import InlineSvg from "vue-inline-svg";
import startSvg from "@/assets/homeSvg/start.svg";
import { useBackgroundSvgStore } from "./store/backgroundSvg";
import Header from "./components/Header.vue";
import { SEND, ws } from "@/plugins/webSockets";
import { onMounted } from "vue";

onMounted(() => {});

let bgSvg = useBackgroundSvgStore();
</script>

<template>
  <div class="absolute z-20" :style="bgSvg.translateX()">
    <InlineSvg :src="startSvg" />
  </div>
  <Header />
  <div class="row-start-2">
    <router-view v-slot="{ Component }">
      <transition name="list">
        <component :is="Component" />
      </transition>
    </router-view>
  </div>
</template>

<style scoped></style>
