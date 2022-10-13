<template>
  <div class="px-8 mx-auto w-3/5 top-2/4">
    <div
      class="absolute left-0 right-0 top-2/4 mx-auto w-3/5 md:w-[350px] lg:w-[430px] xl:w-[500px] bg-sky-200 shadow-sky-500 shadow sm:shadow-sm px-3 py-2 opacity-90"
    >
      <div class="grid grid-cols-3 grid-rows-4 gap-2 h-full px-4 tfont">
        <h3
          class="col-span-3 row-start-1 t-xl text-center lg:text-lg xl:text-2xl 2xl:text-3xl text-black"
        >
          Select minutes
        </h3>
        <button
          @click="store.setMinute(1)"
          class="col-start-1 row-span-3 text-xl hover:bg-orange-300 active:bg-sky-300 lg:text-lg xl:text-2xl 2xl:text-3xl bg-sky-100 rounded sm:rounded-sm shadow-sky-300 shadow sm:shadow-sm md:shadow-md"
        >
          1
        </button>
        <button
          @click="store.setMinute(2)"
          class="col-start-2 row-span-3 text-xl hover:bg-orange-300 active:bg-sky-300 lg:text-lg xl:text-2xl 2xl:text-3xl bg-sky-100 rounded sm:rounded-sm shadow-sky-300 shadow sm:shadow-sm md:shadow-md"
        >
          2
        </button>
        <button
          @click="store.setMinute(3)"
          class="col-start-3 row-span-3 text-xl hover:bg-orange-300 active:bg-sky-300 lg:text-lg xl:text-2xl 2xl:text-3xl bg-sky-100 rounded sm:rounded-sm shadow-sky-300 shadow sm:shadow-sm md:shadow-md"
        >
          3
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRequestStore } from "@/store/createGameStore";

const store = useRequestStore();
</script>

<style scoped>
.tfont {
  font-family: TitilliumRegular;
}
</style>
