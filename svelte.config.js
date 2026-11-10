import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  compilerOptions: {
    // CSS dei componenti inserito dal JavaScript: evita i moduli CSS virtuali,
    // che in sviluppo a volte non si caricano e lasciano la pagina senza stili
    css: "injected",
  },
  kit: {
    // Modalità SPA: tutte le rotte servite da index.html
    adapter: adapter({ fallback: "index.html" }),
  },
};

export default config;
