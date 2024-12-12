import { invoke } from "@tauri-apps/api/core";
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

export async function openWindow(windowName: WindowName) {
  switch (windowName) {
    case "about":
      // await createAboutWindow();
      await invoke("open_new_window", { windowName });
      break;
    case "settings":
      await invoke("open_new_window", { windowName });
      break;
    default:
      break;
  }
}
