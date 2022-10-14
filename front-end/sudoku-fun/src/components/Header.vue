<template>
  <div v-if="inRoutes()" class="col-start-1 row-start-1">
    <div class="flex justify-center">
      <router-link to="/">
        <header class="tf py-1 text-xl">1Sudoku1.fun</header>
      </router-link>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";

function inRoutes(): boolean {
  let fullPath = router.value.fullPath;
  if (fullPath.startsWith("/game")) {
    return true
  }
  return false
}

const router = useRouter().currentRoute;
</script>

<style scoped>
.tf {
  font-family: LabtopSuperWide;
}
</style>
