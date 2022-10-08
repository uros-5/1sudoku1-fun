<template>
  <div
    class="grid grid-cols-12 all-rows md:grid-cols-6 grid-rows-6 md:grid-rows-4 gap-1"
  >
    <GameClock />
    <GameBoard />
    <GameNumbers />
    <GameButtons />
    <GamePlayers />
  </div>
</template>

<style>
.all-rows {
  grid-template-rows: 0.1fr 3fr 0.75fr 0.35fr;
}

@media (min-width: 540px) {
  .all-rows {
    grid-template-rows: 0.1fr 2.6fr 0.65fr 1.4fr;
  }
}

@media (min-width: 640px) {
  .all-rows {
    grid-template-rows: 0.1fr 2.8fr 0.65fr 0.4fr;
  }
}

@media (min-width: 768px) {
  .all-rows {
    grid-template-rows: 0.2fr 0.6fr 0.2fr 0.4fr;
    grid-template-columns: 0.2fr 0.2fr 0.8fr 0.6fr 0.25fr 0.03fr 0.2fr;
  }
}
</style>

<script setup lang="ts">
import GameClock from "@/components/GameClock.vue";
import GameBoard from "@/components/GameBoard.vue";
import GameNumbers from "@/components/GameNumbers.vue";
import GameButtons from "@/components/GameButtons.vue";
import GamePlayers from "@/components/GamePlayers.vue";
</script>
