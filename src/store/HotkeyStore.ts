import { invoke } from "@tauri-apps/api";
import { isRegistered, register } from "@tauri-apps/api/globalShortcut";
import { createRoot, createSignal } from "solid-js";
import { Hotkey, HotkeyEvent } from "../@types";
import { parseShortcut, registerHotkeys } from "../utils/hotkeyRegister";
import AppStore from "./AppStore";

function createHotkeyStore() {
  const [globalHotkeyEvent, setGlobalHotkeyEvent] = createSignal<boolean>(true);
  const [hotkeys, setHotkeys] = createSignal<Hotkey[]>([]);

  const updateHotkey = async (
    hotkey: Hotkey,
    upload: boolean | undefined = true,
  ) => {
    if (upload) await invoke("update_hotkey", { hotkey });
    setHotkeys((prev) =>
      prev.map((h) => (h.id === hotkey.id ? { ...h, ...hotkey } : h)),
    );
  };

  const getHotkey = (event: HotkeyEvent) =>
    hotkeys().find((h) => h.event === event);

  const initHotkeys = async (reg: boolean | undefined = false) => {
    // await unregisterAll();

    const hotkeys = (await invoke<Hotkey[]>("get_hotkeys")).map((h) => ({
      ...h,
      shortcut: parseShortcut(h),
    }));

    setHotkeys(hotkeys);

    // Display and hide the app window
    const windowHotkey = hotkeys.find(
      (h) => h.event === "window_display_toggle",
    );

    if (windowHotkey?.status && !(await isRegistered(windowHotkey.shortcut))) {
      register(windowHotkey.shortcut, () => {
        AppStore.updateSidebarIcons("Recent Clipboards");
        invoke("window_display_toggle");
      }).catch(() => {});
    }

    if (reg) await registerHotkeys(hotkeys);
  };

  return {
    globalHotkeyEvent,
    setGlobalHotkeyEvent,
    hotkeys,
    setHotkeys,
    updateHotkey,
    initHotkeys,
    getHotkey,
  };
}

export default createRoot(createHotkeyStore);
