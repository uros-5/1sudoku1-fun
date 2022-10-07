<template>
  <div v-if="router.fullPath != '/'" class="col-start-1 row-start-1">
    <div class="flex justify-center">
      <router-link to="/">
        <header class="tf py-1 text-xl">1Sudoku1.fun</header>
      </router-link>
      <!--

            <InlineSvg width="250px" :src="logo1" />
            -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";
import InlineSvg from "vue-inline-svg";
import logo1 from "@/assets/homeSvg/logo1.svg";
const router = useRouter().currentRoute;
</script>

<style scoped>
.tf {
  font-family: LabtopSuperWide;
}
</style>
