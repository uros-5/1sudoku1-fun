<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import HelloWorld from './components/HelloWorld.vue'
import Sockette from "sockette";
import InlineSvg from "vue-inline-svg";
import test from "@/assets/homeSvg/test.json";
import startSvg from "@/assets/homeSvg/start.svg";
import anime, { AnimeInstance } from "animejs";
import { onMounted, Ref, ref } from 'vue';

const ws = new Sockette('ws://localhost:9000/ws/', {
  timeout: 5e3,
  maxAttempts: 0,
  onopen: e => console.log('Connected!', e),
  onmessage: e => console.log('Received:', e),
  onreconnect: e => console.log('Reconnecting...', e),
  onmaximum: e => console.log('Stop Attempting!', e),
  onclose: e => console.log('Closed!', e),
  onerror: e => console.log('Error:', e)
});
console.log(ws);
console.log(test);

const m: Ref<AnimeInstance | undefined> = ref();


function mo() {
  let morphing = anime({
    targets: "#rect2",
    d: test.all,
    easing: "linear",
    loop: false,
    duration: 1500
  });
  m.value = morphing;
}

function reverse() {
  m.value!.reverse();
  m.value!.play();
}

onMounted( () => {
  setTimeout(mo, 1000);

});

</script>

<template>
  <div class="absolute">

    <InlineSvg @click="reverse()"  :src="startSvg"/>
  </div>
 
</template>

<style scoped>

</style>
