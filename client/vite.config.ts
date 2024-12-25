import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [wasm(), sveltekit()],
  resolve: {
    alias: {
      "@dojoengine/sdk-svelte": "../dist/index.js",
      $lib: "/src/lib",
    },
  },
});
