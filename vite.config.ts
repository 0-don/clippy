import tailwindcss from "@tailwindcss/vite";
// @ts-ignore
import { join, resolve } from "node:path";
import { defineConfig } from "vite";
import checker from "vite-plugin-checker";
import solidPlugin from "vite-plugin-solid";
// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    tailwindcss(),
    solidPlugin(),
    checker({
      typescript: true,
    }),
  ],
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,

    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  build: {
    rollupOptions: {
      input: {
        main: join(resolve(), "index.html"),
        about: join(resolve(), "pages/about.html"),
        settings: join(resolve(), "pages/settings.html"),
      },
    },
  },
}));
