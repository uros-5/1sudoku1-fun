<template>
    <p></p>
</template>


<script setup lang="ts">
import { SEND, ws } from '@/plugins/webSockets';
import router from '@/router';
import { useBackgroundSvgStore } from '@/store/backgroundSvg';
import { onMounted } from 'vue';

onMounted(() => {
    let id = router.currentRoute.value.params["id"];
    SEND({ t: "accept_game", "game_id": id });
    useBackgroundSvgStore().$state.active = false;
})

</script>

<style>

</style>