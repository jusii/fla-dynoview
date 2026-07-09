import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  // Tauri's webviews are modern (WebView2 / WKWebView / WebKitGTK) — allow
  // top-level await and other recent syntax.
  build: {
    target: "esnext",
  },
  // Tauri expects a fixed port and its own clear-screen behaviour.
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? { protocol: "ws", host, port: 1421 }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
