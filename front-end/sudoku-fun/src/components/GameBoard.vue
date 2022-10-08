<template>
  <div class="col-start-1 col-span-12 row-start-2 md:col-start-2 md:col-span-2 md:row-start-1 md:row-span-3 p-2">

    <div
      class="game-wrapper relative z-10 select-none md:w-[350px] md:h-[350px] lg:w-[430px] lg:h-[430px] xl:w-[500px] xl:h-[500px] 2xl:w-[660px] 2xl:h-[660px]">
      <div class="absolute z-2 w-full h-full">
        <div
          class="border-2 border-neutral-900 w-full h-full bg-sky-100 shadow-sky-400  lg:shadow-sky-800 shadow-sm lg:shadow-md">
          <div class="flex justify-between flex-wrap h-full sudoku-board" @click="cl">
            <div
              class="w-[11.11%] h-[11.11%] border-[0.15px] md:border[1px] lg:border-[1.3px] xl:border-[1.8px] border-sky-400 sudoku-item"
              :class="{ bottomBorder: bottomBorder(i) }" :data-field="i" v-for="(v, i) in defaultLine" :key="i">
              <div class="text-2xl text-black bg-contain h-full w-full pointer-events-none" :style="currentOpacity(i)">
                <InlineSvg :src="numbers[parseInt(v) - 1]" v-if="v != '.'" />
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
import { onMounted, Ref, ref } from "vue";
import { generateSudokuItems, SudokuItem } from "@/plugins/sudokuItems";


const numbers = [svg1, svg2, svg3, svg4, svg5, svg6, svg7, svg8, svg9];
const defaultLine = "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";
const myLine = ref(String(defaultLine));
const selected = ref(-1);
const others: Ref<number[]> = ref([]);
const sudokuItems: Ref<Array<SudokuItem>> = ref([]);

function bottomBorder(index: number): boolean {
  if (index >= 18 && index <= 18 + 8) {
    return true;
  } else if (index >= 45 && index <= 45 + 8) {
    return true;
  }
  return false;
}

function cl(event: MouseEvent) {
  let target = event.target as HTMLDivElement;
  let field = target.attributes.getNamedItem("data-field");
  if (field) {
    let value = parseInt(field.value);
    selected.value = value;
    let block = fieldBlock(value);
    let vertical = fieldVertical(value);
    let horizontal = fieldHorizontal(value);
    others.value = [...vertical, ...horizontal, ...block];
    console.log(others.value);
  }
}

function fieldBlock(index: number): number[] {
  let block = sudokuItems.value.at(index)?.block;
  let items: number[] = []
  sudokuItems.value.forEach((value, index2) => {
    if (value.block == block && index2 != index) {
      items.push(index2);
    }
  })
  return items;
}

function fieldVertical(index: number): number[] {
  let vertical = sudokuItems.value.at(index)?.vertical;
  let items: number[] = []
  sudokuItems.value.forEach((value, index2) => {
    if (value.vertical == vertical && index2 != index) {
      items.push(index2);
    }
  });

  return items;
}

function fieldNumbers(index: number): number[] {
  let itemValue = sudokuItems.value.at(index)?.value;
  let items: number[] = []
  sudokuItems.value.forEach((value, index2) => {
    if (value.value == itemValue && index2 != index) {
      items.push(index2);
    }
  });
  return items;
}

function fieldHorizontal(index: number) {
  let itemValue = sudokuItems.value.at(index)?.horizontal;
  let items: number[] = []
  sudokuItems.value.forEach((value, index2) => {
    if (value.horizontal == itemValue && index2 != index) {
      items.push(index2);
    }
  });

  return items;
}

function currentOpacity(index: number): string {
  let item = others.value.find(item => item == index);
  let opacity = 1;
  let bg = "none";
  if (item || item == 0) {
    opacity = 0.7;
    bg = "rgba(172, 201, 178, 0.4)";
  }
  return `--currentOpacity: ${opacity}; background-color: ${bg};`;
}

onMounted(async () => {
  sudokuItems.value = await generateSudokuItems();
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

@media (min-width:1280px) {
  .game-wrapper::after {
    display: none;
    padding-bottom: 0%;
    content: "";
  }
}
</style>
