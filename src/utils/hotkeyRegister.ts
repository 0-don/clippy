import { invoke } from "@tauri-apps/api";
import {
  isRegistered,
  register,
  registerAll,
  unregister,
  unregisterAll,
} from "@tauri-apps/api/globalShortcut";
import { Hotkey } from "../@types";
import ClipboardStore from "../store/ClipboardStore";
import HotkeyStore from "../store/HotkeyStore";
import SettingsStore from "../store/SettingsStore";
import { CLIPBOARD_HOTKEYS } from "./constants";

export const parseShortcut = (hotkey: Hotkey) => {
  const { ctrl, alt, shift, key } = hotkey;
  const modifiers = [];
  if (ctrl) modifiers.push("CommandOrControl");
  if (alt) modifiers.push("Alt");
  if (shift) modifiers.push("Shift");
  return `${modifiers.join("+")}+${key.toUpperCase()}`;
};

export async function registerHotkeys(hotkeys: Hotkey[]) {
  const { setGlobalHotkeyEvent } = HotkeyStore;
  const { getCurrentTab } = SettingsStore;
  const { clipboards } = ClipboardStore;
  await unregisterAll();

  // ############################################
  setGlobalHotkeyEvent(true);
  // ############################################

  // Display and hide the app window
  const mainHotkey = hotkeys.find((h) => h.event === "window_display_toggle");

  if (
    mainHotkey &&
    mainHotkey?.shortcut &&
    !(await isRegistered(mainHotkey.shortcut)) &&
    mainHotkey.status
  ) {
    try {
      await register(mainHotkey.shortcut, () =>
        invoke("window_display_toggle")
      );
    } catch (_) {}
  }

  const leftOverHotkeys = hotkeys.filter(
    (h) => h.event !== "window_display_toggle"
  );

  // copy to clipboard
  await registerAll(
    CLIPBOARD_HOTKEYS,
    async (num) =>
      await invoke("copy_clipboard", { id: clipboards()[Number(num) - 1].id })
  );

  for (const hotkey of leftOverHotkeys) {
  }

  setTimeout(async () => {
    for (const key of CLIPBOARD_HOTKEYS) {
      try {
        await unregister(key);
      } catch (_) {}
    }

    for (const hotkey of leftOverHotkeys) {
      try {
        await unregister(hotkey.shortcut);
      } catch (_) {}
    }
    setGlobalHotkeyEvent(false);
  }, 5000);
}
