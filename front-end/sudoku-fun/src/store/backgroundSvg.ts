import anime from "animejs";
import { AnimeInstance } from "animejs";
import { defineStore } from "pinia";
import test from "@/assets/homeSvg/test.json";
import { useRouter } from "vue-router";
import { resolve } from "path";

export const useBackgroundSvgStore = defineStore("bg", {
  state: (): BgStore => {
    return { active: false, anime: undefined };
  },
  actions: {
    translateX(): string {
      let x = this.$state.active ? 0 : -100;
      return `transform: translateX(${x}%);`;
    },
    toggleActive() {
      let current = this.$state.active;
      this.$state.active = !current;
    },
    activate(): Promise<boolean> {
      return new Promise((resolve) => {
        this.$state.active = true;
        let morphing = anime({});
        let self = this;
        let cnt = 0;
        morphing = anime({
          targets: "#rect2",
          d: test.all,
          easing: "linear",
          loop: false,
          duration: 800,
        });
        morphing.changeComplete = function () {
          setTimeout(() => {
            if (cnt == 0) {
              morphing.reverse();
              morphing.play();
              resolve(true);
              cnt = 1;
            } else {
              self.toggleActive();
              return;
            }
          }, 100);
        };
      });
    },
    isActive(): boolean {
      return this.$state.active;
    },
  },
});

export interface BgStore {
  active: boolean;
  anime: AnimeInstance | undefined;
}
