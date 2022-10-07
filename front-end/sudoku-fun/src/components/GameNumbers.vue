<template>
  <div
    class="col-start-3 col-end-9 row-start-3 md:col-start-4 md:col-span-1 md:row-start-2"
  >
    <div
      class="bg-sky-100 min-h-full grid grid-cols-5 md:grid-cols-3 ignore-last justify-center border-x-0 border-t-0 border-r-0 border-2 border-sky-300 shadow-sky-300 shadow"
    >
      <button
        v-for="i in numbers"
        class="text-4xl lg:text-5xl xl:text-6xl tfont text-sky-900 dark:text-orange-200 border-r-[1px]"
      >
        {{ i }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, "x"];
</script>

<style scoped>
@media (min-width: 768px) {
  .ignore-last button:last-of-type {
    display: none;
  }
}

.tfont {
  font-family: TitilliumRegular;
}
</style>
