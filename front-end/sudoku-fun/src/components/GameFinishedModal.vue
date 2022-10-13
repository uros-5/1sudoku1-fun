<template>

 <div class="px-8 mx-auto w-3/5 top-2/4 z-30">
    <div
      class="absolute left-0 right-0 top-2/4 mx-auto w-3/5 md:w-[350px] lg:w-[430px] xl:w-[500px] bg-sky-200 shadow-sky-500 shadow sm:shadow-sm px-3 py-2 opacity-90"
    >
      <div class="grid grid-cols-3 grid-rows-2 gap-2 h-full px-4 tfont">
        <h3
          class="col-span-3 row-start-1 t-xl text-center lg:text-lg xl:text-2xl 2xl:text-3xl text-black"
        >
           {{ statusMsg() }} 
        </h3>
        <h4
          class="col-span-3 row-start-2 t-xl text-center lg:text-lg xl:text-2xl 2xl:text-3xl text-black"
        >
           {{ resultMsg() }} 
        </h4>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useSudokuStore } from '@/store/sudokuStore';


const store = useSudokuStore();

function statusMsg(): string {
    let myIndex = store.myIndex();
    let otherIndex = + Boolean(!myIndex);
    let players = store.$state.game.players;
    let status = store.$state.game.status;
    if (status == 3) {
        return "";
    }
    else if (status == 2) {
        return "Draw!"
    }
    else if (status == myIndex) {
        return "You won!"
    }
    else if (status == myIndex + 4) {
      return "You resigned!"
    }
    else if (status == otherIndex + 4) {
      return `${players[otherIndex]} resigned!`
    }
    return "You lost";
}

function resultMsg(): string {
    let score = store.$state.game.score;
    let myIndex = store.myIndex();
    let otherIndex = + Boolean(!myIndex);
    return `${score[myIndex]} - ${score[otherIndex]}`;
}

</script>

<style scoped>
.tfont {
  font-family: TitilliumRegular;
}

</style>