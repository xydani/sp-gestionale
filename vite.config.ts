import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vitest/config";

const tauriDevHost = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [sveltekit()],
  // Non nascondere gli errori di compilazione di Rust
  clearScreen: false,
  server: {
    // Tauri si aspetta una porta fissa
    port: 1420,
    strictPort: true,
    host: tauriDevHost || "127.0.0.1",
    hmr: tauriDevHost ? { protocol: "ws", host: tauriDevHost, port: 1421 } : undefined,
    watch: { ignored: ["**/src-tauri/**"] },
  },
  test: {
    include: ["src/**/*.test.ts"],
    environment: "node",
  },
});
