import { WebviewWindow } from "@tauri-apps/api/window";
import { WindowName } from "../@types";

export function formatBytes(bytes: number, decimals = 2) {
  if (bytes === 0) return "0 Bytes";

  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`;
}

export async function sleep(milis: number) {
  return new Promise((resolve) => setTimeout(resolve, milis));
}

export function createAboutWindow() {
  new WebviewWindow("about", {
    url: "./pages/about.html",
    title: "About",
    width: 375,
    height: 600,
    alwaysOnTop: true,
  });
}

export function createSettingsWindow() {
  new WebviewWindow("settings", {
    url: "./pages/settings.html",
    title: "Settings",
    height: 425,
    width: 500,
    alwaysOnTop: true,
  });
}

export function openWindow(windowName: WindowName) {
  switch (windowName) {
    case "about":
      createAboutWindow();
      break;
    case "settings":
      createSettingsWindow();
      break;
    default:
      break;
  }
}
