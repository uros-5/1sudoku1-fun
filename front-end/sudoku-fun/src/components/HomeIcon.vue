<template>
  <InlineSvg :src="logo2Svg" />
</template>

<script setup lang="ts">
import randomItem from "random-item";
import { Ref, ref } from "vue";
import InlineSvg from "vue-inline-svg";
import logo2Svg from "@/assets/homeSvg/logo2.svg";
import { useBackgroundSvgStore } from "@/store/backgroundSvg";
const bg = useBackgroundSvgStore();

const numInterval: Ref<NodeJS.Timer | undefined> = ref();
let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9];
let opacity = [0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1];

numInterval.value = setInterval(() => {
  if (bg.isActive()) {
    clearInterval(numInterval.value);
    return;
  }
  let items = randomItem.multiple(nums, 3);
  let opacityR = randomItem.multiple(opacity, 3);
  items.forEach((v, i) => {
    q(v, opacityR[i]);
  });
  setTimeout(() => {
    items.forEach((v) => {
      q(v, 0.2);
    });
  }, 900);
}, 1000);

function q(v: number, opacity: number) {
  let query = document.querySelector(`#text${v}`) as HTMLElement | null;
  if (query) {
    query.style.fillOpacity = `${opacity}`;
  }
}
</script>

<style></style>
