<template>
  <main
    class="w-full md:min-h-40 xl:min-h-screen flex flex-col justify-center items-center gap-y-32 md:gap-42 content-around p-9"
    :class="{ 'opacity-40 select-none': requestStore.modalActive() }"
    ref="main"
  >
    <HomeIcon class="mt-10 w-auto sm:w-4/5 md:w-3/5 lg:w-2/5 xl:w-2/5" />
    <button
      @click="click"
      :disabled="requestStore.modalActive()"
      class="text-center tfont primary-text rounded-3xl bg-orange-200 text-sky-900 hover:bg-orange-300"
      :class="{ 'cursor-not-allowed': requestStore.modalActive() }"
    >
     {{ buttonText() }} 
    </button>

    <Teleport to="#app">
      <CreateGameModal v-if="requestStore.modalActive() == true" />
    </Teleport>

    <h4 class="text-center tfont primary-text">{{ gameStore.gameCount() }} games active</h4>

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
import CreateGameModal from "@/components/CreateGameModal.vue";
import { Ref, ref, watch, onMounted } from "vue";
import { useRequestStore } from "@/store/createGameStore";
import { SEND } from "@/plugins/webSockets";
import { useSudokuStore } from "@/store/sudokuStore";
import router from "@/router";
import { frontend } from "@/plugins/getBackend";

const requestStore = useRequestStore();
const gameStore = useSudokuStore();
const main: Ref<HTMLElement | null> = ref(null);

function click(event: MouseEvent) {
  let requestId = requestStore.id();
  let gameId = gameStore.id();
  if (requestId == "" && gameId == "") {
    requestStore.toggleModal();
  }
  else if (requestId != "" && gameId == "") {
    let url = frontend();
    navigator.clipboard.writeText(`${url}g/${requestId}`);
  }
  else if (requestId == "" && gameId != "") {
    router.push(`/game/${gameId}`);
  }
  else {
    return "";
  }
  
}

function buttonText(): string {
  let requestId = requestStore.id();
  let gameId = gameStore.id();
  if (requestId == "" && gameId == "") {
    return "Create game";
  }
  else if (requestId != "" && gameId == "") {
    return "Copy game url";
  }
  else if (requestId == "" && gameId != "") {
    return "Open live game";
  }
  else {
    return "";
  }
}

watch(main, (newValue) => {
  requestStore.setMain(newValue);
});

onMounted(() => {
  SEND({ t: "request_url" });
  SEND({ t: "game_url" });
});
</script>

<style scoped>
.tfont {
  font-family: TitilliumThin;
  font-weight: bold;
}
</style>
