<template>
    <p></p>
</template>


<script setup lang="ts">
import { SEND, ws } from '@/plugins/webSockets';
import router from '@/router';

let id = router.currentRoute.value.params["id"];
SEND({t: "accept_game", "game_id": id});

</script>

<style></style>