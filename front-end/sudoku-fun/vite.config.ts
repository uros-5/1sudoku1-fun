import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "path";
import { ViteRsw } from 'vite-plugin-rsw';

// https://vitejs.dev/config/
export default defineConfig({
  define: {
    "process.env": process.env,
  },
  plugins: [vue()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  optimizeDeps: { exclude: ["sudoku-wasm"] },
});
