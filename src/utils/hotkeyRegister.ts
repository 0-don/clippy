import { app, invoke } from "@tauri-apps/api";
import { register,unregisterAll } from "@tauri-apps/api/globalShortcut";
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

  // Display and hide the app window
  const mainHotkey = hotkeys.find((h) => h.event === "window_display_toggle");
  if (mainHotkey && mainHotkey.status) {
    await register(mainHotkey.shortcut, async () => {
      const isVisible = await appWindow.isVisible();
      if (isVisible) {
        await appWindow.hide();
      } else {
        await appWindow.show();
        await appWindow.setFocus();
        await invoke("window_on_mouse");
        // move_window(Position.TrayCenter);
      }
    });
  }
  // ############################################
}
