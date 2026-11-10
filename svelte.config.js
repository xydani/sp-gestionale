import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    // Modalità SPA: tutte le rotte servite da index.html
    adapter: adapter({ fallback: "index.html" }),
  },
};

export default config;
