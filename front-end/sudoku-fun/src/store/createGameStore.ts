import { SEND } from "@/plugins/webSockets";
import { createGameStore, Minute } from "@/storeTypes";
import { defineStore } from "pinia";
import { Ref, ref } from "vue";

export const useRequestStore = defineStore("createGame", {
  state: (): createGameStore => {
    return {
      modalActive: false,
      minute: 1,
      main: ref(null),
      id: "",
    };
  },
  actions: {
    setMinute(minute: Minute) {
      this.$state.minute = minute;
      this.hideModal();
      SEND({ t: "create_game", minute: minute });
    },
    hideModal(_?: MouseEvent) {
      if (this.modalActive()) {
        this.$state.modalActive = false;
        this.$state.main?.removeEventListener("click", this.hideModal);
      }
    },
    toggleModal() {
      this.$state.modalActive = !this.$state.modalActive;
      setTimeout(() => {
        this.$state.main?.addEventListener("click", this.hideModal);
      }, 400);
    },
    setMain(main: null | HTMLElement) {
      this.$state.main = main;
    },
    modalActive(): boolean {
      return this.$state.modalActive;
    },
    id(): string {
      return this.$state.id;
    },
    setId(id: string) {
      this.$state.id = id;
    },
  },
});
