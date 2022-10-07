<template>
  <div
    class="col-start-1 col-span-12 row-start-2 md:col-start-2 md:col-span2 md:row-start-1 md:row-span-3 p-2"
  >
    <!--
      md:w-[500px] md:h-[500px]
    -->
    <div
      class="game-wrapper relative -z-10 md:w-[350px] md:h-[350px] lg:w-[430px] lg:h-[430px] xl:w-[500px] xl:h-[500px] 2xl:w-[660px] 2xl:h-[660px]"
    >
      <div class="absolute z-2 w-full h-full">
        <div class="border-2 border-neutral-900 w-full h-full bg-neutral-200">
          <div class="flex justify-between flex-wrap h-full sudoku-board">
            <div
              class="w-[11.11%] h-[11.11%] border-[0.15px] md:border[1px] lg:border-[1.3px] xl:border-[1.8px] border-white sudoku-item"
              :class="{ bottomBorder: bottomBorder(i) }"
              v-for="(v, i) in line"
              :key="v"
            >
              <div class="text-2xl text-black bg-contain h-full w-full">
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

const numbers = [svg1, svg2, svg3, svg4, svg5, svg6, svg7, svg8, svg9];
const line =
  "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";

function bottomBorder(index: number): boolean {
  if (index >= 18 && index <= 18 + 8) {
    return true;
  } else if (index >= 45 && index <= 45 + 8) {
    return true;
  }
  return false;
}
</script>

<style scoped>
.sudoku-board {
}

.sudoku-item {
}

.bottomBorder {
  border-bottom: 1.8px solid black !important;
}

.sudoku-board .sudoku-item:nth-of-type(3n) {
  border-right: 1.8px solid black !important;
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
</style>
