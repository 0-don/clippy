import { join, resolve } from "path";
import { defineConfig } from "vite";
import checker from "vite-plugin-checker";
import solidPlugin from "vite-plugin-solid";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
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
  },
  // 3. to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    rollupOptions: {
      input: {
        main: join(resolve(), "index.html"),
        about: join(resolve(), "pages/about.html"),
        settings: join(resolve(), "pages/settings.html"),
      },
    },
  },
});
