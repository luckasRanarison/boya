import { defineConfig } from "vite";
import path from "path";
import react from "@vitejs/plugin-react";
import { visualizer } from "rollup-plugin-visualizer";
import topLevelAwait from "vite-plugin-top-level-await";
import wasm from "vite-plugin-wasm";
import svgr from "vite-plugin-svgr";
import pkg from "./package.json";

// https://vite.dev/config/
export default defineConfig({
  base: "/boya",
  plugins: [react(), svgr(), wasm(), topLevelAwait(), visualizer()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  define: {
    "import.meta.env.APP_VERSION": JSON.stringify(pkg.version),
  },
});
