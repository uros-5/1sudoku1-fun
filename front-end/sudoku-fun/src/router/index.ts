import { useBackgroundSvgStore } from "@/store/backgroundSvg";
import { createRouter, createWebHistory, RouteRecordRaw } from "vue-router";
const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    component: () => import(/* webpackChunkName: "Home" */ "@/pages/Home.vue"),
  },
  {
    path: "/game/:id",
    component: () =>
      import(/* webpackChunkName: "LiveGame" */ "@/pages/LiveGame.vue"),
  },
  {
    path: "/g/:id",
    component: () => import(/* webpackChunkName: "NewGame" */ "@/pages/NewGame.vue")
  }
];

//

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes,
});

router.beforeEach(async (to, from) => {
  //if (from.fullPath == "/" && to.fullPath.startsWith("/game")) {
  const store = useBackgroundSvgStore();
  let b = await store.activate();
  return b;
});

export default router;
