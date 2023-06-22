import { invoke } from "@tauri-apps/api";
import {
  isRegistered,
  register,
  registerAll,
  unregister,
  unregisterAll,
} from "@tauri-apps/api/globalShortcut";
import { Hotkey } from "../@types";
import AppStore from "../store/AppStore";
import ClipboardStore from "../store/ClipboardStore";
import HotkeyStore from "../store/HotkeyStore";
import { CLIPBOARD_HOTKEYS } from "./constants";

export const parseShortcut = (hotkey: Hotkey) => {
  const { ctrl, alt, shift, key } = hotkey;
  const modifiers = [];
  if (ctrl) modifiers.push("CommandOrControl");
  if (alt) modifiers.push("Alt");
  if (shift) modifiers.push("Shift");
  return `${modifiers.join("+")}${
    modifiers.length ? "+" : ""
  }${key.toUpperCase()}`;
};

export let timer: NodeJS.Timeout | undefined;

export async function registerHotkeys(hotkeys: Hotkey[]) {
  const { setGlobalHotkeyEvent } = HotkeyStore;
  const { getCurrentSidebarIcon } = AppStore;
  const { clipboards, clipboardRef } = ClipboardStore;
  await unregisterAll();

  // ############################################
  setGlobalHotkeyEvent(true);
  // ############################################

  // Display and hide the app window
  const mainHotkey = hotkeys.find((h) => h.event === "window_display_toggle");
  if (mainHotkey?.status && !(await isRegistered(mainHotkey.shortcut))) {
    try {
      await register(mainHotkey.shortcut, () =>
        invoke("window_display_toggle")
      );
    } catch (_) {}
  }

  // copy to clipboard
  await registerAll(CLIPBOARD_HOTKEYS, async (num) => {
    await invoke("copy_clipboard", { id: clipboards()[Number(num) - 1].id });
    removeAllHotkeyListeners();
  });

  const scrollToTop = hotkeys.find((h) => h.event === "scroll_to_top");

  if (scrollToTop?.status && getCurrentSidebarIcon()?.name !== "View more") {
    await register(scrollToTop.shortcut, () => clipboardRef()!.scrollTo(0, 0));
  }

  // for (const hotkey of hotkeys) {
  //   console.log(hotkey.name);
  // }

  timer = setTimeout(removeAllHotkeyListeners, 5000);
}

export const removeAllHotkeyListeners = async () => {
  const { hotkeys, setGlobalHotkeyEvent } = HotkeyStore;
  for (const key of CLIPBOARD_HOTKEYS) {
    try {
      await unregister(key);
    } catch (_) {}
  }

  for (const hotkey of hotkeys()) {
    if (hotkey.event === "window_display_toggle") continue;
    try {
      await unregister(hotkey.shortcut);
    } catch (_) {}
  }
  setGlobalHotkeyEvent(false);
  clearTimeout(timer);
};
