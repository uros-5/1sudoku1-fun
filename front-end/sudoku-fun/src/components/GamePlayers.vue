<template>
  <div
    class="col-start-1 col-end-3 row-start-4 md:col-start-3 md:col-span-1 md:row-start-4"
  >
    <div class="flex flex-col p-3">
      <h1 class="text-2xl tf md:text-xl lg:text-2xl xl:text-3xl cursor-default">
        {{ store.$state.game.players[0]}} <span><b>{{ me(0) }}</b></span>
      </h1>
      <h1 class="text-2xl tf md:text-xl lg:text-2xl xl:text-3xl cursor-default">
        {{ store.$state.game.players[1]}} <span><b>{{ me(1) }}</b></span>
      </h1>
      <h5 class="text-xl md:text-xs lg:text-xl xl:text-3xl  cursor-default">
        {{ new Date(store.$state.game.clock.last_click).toLocaleString() }}
      </h5>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSudokuStore } from '@/store/sudokuStore';

const store = useSudokuStore();

function me(index: number): string {
  if (store.$state.game.players[index] == store.$state.username) {
    return "(me)";
  }
  return "";
}

</script>

<style scoped>
.tf {
  font-family: TitilliumThin;
}
</style>
