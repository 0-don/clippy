import { register, unregisterAll } from "@tauri-apps/api/globalShortcut";
import { appWindow } from "@tauri-apps/api/window";
import { Hotkey } from "../@types";

export const parseShortcut = (hotkey: Hotkey) => {
  const { ctrl, alt, shift, key } = hotkey;
  const modifiers = [];
  if (ctrl) modifiers.push("CommandOrControl");
  if (alt) modifiers.push("Alt");
  if (shift) modifiers.push("Shift");
  return `${modifiers.join("+")}+${key.toUpperCase()}`;
};

export async function registerHotkeys(hotkeys: Hotkey[]) {
  await unregisterAll();
  const mainHotkey = hotkeys.find((h) => h.event === "window_display_toggle");
  if (mainHotkey && mainHotkey.status) {
    await register(mainHotkey.shortcut, async () => {
      const minimized = await appWindow.isMinimized();
      if (minimized) {
        await appWindow.unminimize();
      } else {
        await appWindow.minimize();
      }
    });
  }
}
