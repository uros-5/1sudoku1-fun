<template>
  <Teleport to="body">
    <span style="display: none">{{ store.myNumber() }}</span>
  </Teleport>
  <div
    class="col-start-1 col-span-12 row-start-2 md:col-start-2 md:col-span-2 md:row-start-1 md:row-span-3 p-2"
  >
    <div
      class="game-wrapper relative z-10 select-none md:w-[350px] md:h-[350px] lg:w-[430px] lg:h-[430px] xl:w-[500px] xl:h-[500px] 2xl:w-[660px] 2xl:h-[660px]"
    >
      <div class="absolute z-2 w-full h-full">
        <div
          class="border-2 border-neutral-900 w-full h-full bg-sky-100 shadow-sky-400 lg:shadow-sky-800 shadow-sm lg:shadow-md"
        >
          <div
            class="flex justify-between flex-wrap h-full sudoku-board"
            @click="store.click"
            @contextmenu.prevent="store.contextMenu"
          >
            <div
              class="w-[11.11%] h-[11.11%] border-[0.15px] md:border[1px] lg:border-[1.3px] xl:border-[1.8px] border-sky-400 sudoku-item"
              :class="{ bottomBorder: store.bottomBorder(i) }"
              :data-field="i"
              v-for="(v, i) in store.defaultLine()"
              :key="i + 0"
            >
              <div
                class="text-2xl text-black bg-contain h-full w-full pointer-events-none"
                :style="store.currentOpacity(i)"
              >
                <InlineSvg :src="svgPath(v)" v-if="v != '.'" :id="v" />
                <InlineSvg
                  :src="svgPath(store.myLineAt(i))"
                  style="--currentColor: rgb(3 105 161)"
                  v-else-if="!store.isEmpty(i)"
                  :id="v"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import InlineSvg from "vue-inline-svg";
import svg1 from "@/assets/numbersSvg/num1.svg";
import svg2 from "@/assets/numbersSvg/num2.svg";
import svg3 from "@/assets/numbersSvg/num3.svg";
import svg4 from "@/assets/numbersSvg/num4.svg";
import svg5 from "@/assets/numbersSvg/num5.svg";
import svg6 from "@/assets/numbersSvg/num6.svg";
import svg7 from "@/assets/numbersSvg/num7.svg";
import svg8 from "@/assets/numbersSvg/num8.svg";
import svg9 from "@/assets/numbersSvg/num9.svg";
import { onMounted, onUnmounted } from "vue";
import { useSudokuStore } from "@/store/sudokuStore";

const store = useSudokuStore();

const numbers = [svg1, svg2, svg3, svg4, svg5, svg6, svg7, svg8, svg9];

function svgPath(v: string): string {
  const path = numbers[parseInt(v) - 1];
  return path;
}

onMounted(async () => {
  await store.setSudokuItems();
  store.setKeyboard();
  document.body.addEventListener("keydown", (event) => {
    store.keyDown(event);
  });
});

onUnmounted( () => {
  document.body.removeEventListener("keydown", store.keyDown);
})
</script>

<style scoped>
.bottomBorder {
  border-bottom: 2.8px solid black !important;
}

.sudoku-board .sudoku-item:nth-of-type(3n) {
  border-right: 2.8px solid black !important;
}

.sudoku-board .sudoku-item:nth-child(9n) {
  border-right: none !important;
}

.game-wrapper {
  max-width: 100%;
  max-height: 100%;
}

.game-wrapper::after {
  display: block;
  padding-bottom: 100%;
  content: "";
}

@media (min-width: 1280px) {
  .game-wrapper::after {
    display: none;
    padding-bottom: 0%;
    content: "";
  }
}
</style>
