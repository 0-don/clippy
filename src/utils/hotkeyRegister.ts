import { invoke } from "@tauri-apps/api";
import {
  isRegistered,
  register,
  unregisterAll,
} from "@tauri-apps/api/globalShortcut";
import { appWindow } from "@tauri-apps/api/window";
import { Position, move_window } from "tauri-plugin-positioner-api";
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

  if (
    mainHotkey?.shortcut &&
    !(await isRegistered(mainHotkey?.shortcut)) &&
    mainHotkey &&
    mainHotkey.status
  ) {
    try {
      await register(mainHotkey.shortcut, async () => {
        const isVisible = await appWindow.isVisible();
        if (isVisible) {
          await appWindow.hide();
        } else {
          await appWindow.show();
          await appWindow.setFocus();
          await invoke("window_on_mouse");
          move_window(Position.BottomRight);
        }
      });
    } catch (error) {
      console.error(error);
    }
  }
  // ############################################
}
